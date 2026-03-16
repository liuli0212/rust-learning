use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token, Registry};
use std::collections::HashMap;
use std::future::Future;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll as stdPoll, Waker};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;

/* 
 * --- 自定义运行时驱动机制说明 (Runtime Driving Mechanism) ---
 * 
 * 1. 层级驱动 (Hierarchical Driving):
 *    Executor 并不直接调用底层组件（如 AcceptFuture）的 poll。它只驱动通过 `spawn` 
 *    提交的最外层 Future（如 `server_logic`）。
 * 
 * 2. .await 的传导作用:
 *    当 `server_logic` 执行到 `listener.accept().await` 时，外层 Future 的 poll 
 *    会自动触发内层 `AcceptFuture` 的 poll。这就是“俄罗斯套娃”式的驱动模型。
 * 
 * 3. 阻塞与唤醒闭环:
 *    - 如果 `AcceptFuture` 返回 `Pending`，它会将当前任务的 `Waker` 注册到 `Reactor`。
 *    - 当 `Reactor` 线程监听到网络事件时，调用 `waker.wake()`。
 *    - `Waker` 会通过 `executor_tx` 将整个 `Task`（包含最外层 Future）重新送回 
 *      `Executor` 的就绪队列，触发下一轮从顶至下的 poll。
 * 
 * 
 * Logic架构和时序图: @runtime_architecture.md
 */

/// --- 1. 任务 (Task) ---
struct Task {
    name: String,
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor_tx: Sender<Arc<Task>>,
}

impl Task {
    fn wake(self: &Arc<Self>) {
        println!("[Task: {}] Waking task up, re-enqueuing to executor...", self.name);
        let _ = self.executor_tx.send(self.clone());
    }
}

struct MyWaker {
    task: Arc<Task>,
}

impl std::task::Wake for MyWaker {
    fn wake(self: Arc<Self>) {
        self.task.wake();
    }
}

/// --- 2. 执行器 (Executor) ---
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        // 接收到由 spawn 函数发送的任务，并轮询它们的 future 直到完成。
        while let Ok(task) = self.ready_queue.recv() {
            println!("[Executor] Popped task '{}' from queue, starting to poll...", task.name);
            let mut future: std::sync::MutexGuard<'_, Pin<Box<dyn Future<Output = ()> + Send + 'static>>> = task.future.lock().unwrap();
            // 创建一个 waker 和 context 来轮询 future。
            // waker由 MyWaker 实现，当 future 需要被唤醒时，会调用 MyWaker::wake，将任务重新放回
            // ready_queue。在这个项目中，当 reactor 检测到事件发生时，会调用 MyWaker::wake 将任务重新放回  ready_queue。
            let waker = Waker::from(Arc::new(MyWaker { task: task.clone() }));
            let mut context = Context::from_waker(&waker);
            
            // 调用 future 的 poll 方法，检查是否完成。
            // 如果 future 返回 Pending，说明它还没有完成，等待事件发生时会被唤醒并重新放回 ready_queue。如果是 pending，它需要注册一个 waker 来在事件发生时被唤醒。
            // waker在context中。
            match future.as_mut().poll(&mut context) {
                stdPoll::Ready(_) => println!("[Executor] Task '{}' reached Ready!", task.name),
                stdPoll::Pending => println!("[Executor] Task '{}' returned Pending, waiting for IO events...", task.name),
            }
        }
    }
}

/// --- 3. 反应器 (Reactor) ---
struct Reactor {
    registry: Registry,
    wakers: Mutex<HashMap<Token, Waker>>,
    next_token: AtomicUsize,
}

static REACTOR: OnceLock<Arc<Reactor>> = OnceLock::new();

fn get_reactor() -> &'static Arc<Reactor> {
    REACTOR.get_or_init(|| {
        let poll = Poll::new().unwrap();
        let registry = poll.registry().try_clone().unwrap();
        let reactor = Arc::new(Reactor {
            registry,
            wakers: Mutex::new(HashMap::new()),
            next_token: AtomicUsize::new(1),
        });
        
        let r = reactor.clone();
        thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            let mut p = poll;
            loop {
                // 这里的 poll 由 mio 驱动，它会阻塞直到有事件就绪或超时。 
                // !! 注意它不是 Executor 调用的 poll，而是 mio 驱动的 poll。
                p.poll(&mut events, None).unwrap();
                
                // 有事件就绪，唤醒对应的任务. wake()的实现在 MyWaker 中，它会将整个 Task 重新放回 Executor 的 ready_queue。
                for event in events.iter() {
                    let token = event.token();
                    println!("[Reactor] IO Event ready for token {:?}", token);
                    let mut wakers = r.wakers.lock().unwrap();
                    if let Some(waker) = wakers.remove(&token) {
                        println!("[Reactor] Found waker for token {:?}, triggering wake()", token);
                        waker.wake();
                    }
                }
            }
        });
        reactor
    })
}

impl Reactor {
    fn register(&self, source: &mut dyn mio::event::Source, interest: Interest) -> Token {
        let token = Token(self.next_token.fetch_add(1, Ordering::SeqCst));
        println!("[Reactor] Registered source with token {:?}", token);
        // Registry 是线程安全，可以并发调用 register
        self.registry.register(source, token, interest).unwrap();
        token
    }
}

/// --- 4. 异步 I/O 组件 ---

struct AsyncTcpListener {
    inner: TcpListener,
    token: Token,
}

impl AsyncTcpListener {
    fn bind(addr: SocketAddr) -> Self {
        let mut inner = TcpListener::bind(addr).unwrap();
        let token = get_reactor().register(&mut inner, Interest::READABLE);
        Self { inner, token }
    }

    fn accept(&mut self) -> AcceptFuture {
        AcceptFuture { listener: self }
    }
}

struct AcceptFuture<'a> {
    listener: &'a mut AsyncTcpListener,
}

impl<'a> Future for AcceptFuture<'a> {
    type Output = (AsyncTcpStream, SocketAddr);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> stdPoll<Self::Output> {
        let this = self.get_mut();
        match this.listener.inner.accept() {
            Ok((stream, addr)) => {
                println!("[AcceptFuture] Connection accepted from {:?}", addr);
                stdPoll::Ready((AsyncTcpStream::new(stream), addr))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                println!("[AcceptFuture] WouldBlock, registering waker for token {:?}", this.listener.token);
                /*
                 * 注册 Waker 到 Reactor: 
                 * 1. 此时 Future 返回 Pending，意味着上层调用链也会返回 Pending。
                 * 2. 把当前顶层任务的 Waker 保存在 Reactor 中，对应 mio 的 token。
                 * 3. 当 mio 收到就绪事件后，调用 waker.wake() 将任务重新发回 Executor。
                 */
                get_reactor().wakers.lock().unwrap().insert(this.listener.token, cx.waker().clone());
                // 再次尝试以防竞争
                match this.listener.inner.accept() {
                    Ok((stream, addr)) => {
                        println!("[AcceptFuture] Connection accepted (retry) from {:?}", addr);
                        get_reactor().wakers.lock().unwrap().remove(&this.listener.token);
                        stdPoll::Ready((AsyncTcpStream::new(stream), addr))
                    }
                    Err(_) => stdPoll::Pending,
                }
            }
            Err(e) => panic!("Accept error: {}", e),
        }
    }
}

struct AsyncTcpStream {
    inner: TcpStream,
    token: Token,
}

impl AsyncTcpStream {
    fn new(mut inner: TcpStream) -> Self {
        let token = get_reactor().register(&mut inner, Interest::READABLE | Interest::WRITABLE);
        Self { inner, token }
    }

    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadFuture<'a> {
        ReadFuture { stream: self, buf }
    }

    fn write<'a>(&'a mut self, data: &'a [u8]) -> WriteFuture<'a> {
        WriteFuture { stream: self, data }
    }
}

struct ReadFuture<'a> {
    stream: &'a mut AsyncTcpStream,
    buf: &'a mut [u8],
}

impl<'a> Future for ReadFuture<'a> {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> stdPoll<Self::Output> {
        let this = self.get_mut();
        match this.stream.inner.read(this.buf) {
            Ok(n) => stdPoll::Ready(n),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                println!("[ReadFuture] WouldBlock, registering waker for token {:?}", this.stream.token);
                get_reactor().wakers.lock().unwrap().insert(this.stream.token, cx.waker().clone());
                match this.stream.inner.read(this.buf) {
                    Ok(n) => {
                        get_reactor().wakers.lock().unwrap().remove(&this.stream.token);
                        stdPoll::Ready(n)
                    }
                    Err(_) => stdPoll::Pending,
                }
            }
            Err(e) => panic!("Read error: {}", e),
        }
    }
}

struct WriteFuture<'a> {
    stream: &'a mut AsyncTcpStream,
    data: &'a [u8],
}

impl<'a> Future for WriteFuture<'a> {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> stdPoll<Self::Output> {
        let this = self.get_mut();
        match this.stream.inner.write(this.data) {
            Ok(n) => stdPoll::Ready(n),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                println!("[WriteFuture] WouldBlock, registering waker for token {:?}", this.stream.token);
                get_reactor().wakers.lock().unwrap().insert(this.stream.token, cx.waker().clone());
                match this.stream.inner.write(this.data) {
                    Ok(n) => {
                        get_reactor().wakers.lock().unwrap().remove(&this.stream.token);
                        stdPoll::Ready(n)
                    }
                    Err(_) => stdPoll::Pending,
                }
            }
            Err(e) => panic!("Write error: {}", e),
        }
    }
}

/// --- 5. 启动函数 (Spawner) ---
fn spawn<F>(name: &str, future: F, sender: &Sender<Arc<Task>>)
where
    F: Future<Output = ()> + Send + 'static,
{
    println!("[Spawner] Submitting new task '{}' to executor...", name);
    let task = Arc::new(Task {
        name: name.to_string(),
        future: Mutex::new(Box::pin(future)),
        executor_tx: sender.clone(),
    });
    sender.send(task).unwrap();
}

/// --- 6. 业务逻辑 ---

async fn server_logic(addr: String) {
    let addr: SocketAddr = addr.parse().unwrap();
    let mut listener = AsyncTcpListener::bind(addr);
    println!("[Server] Listening on {}", addr);

    loop {
        let (mut stream, client_addr) = listener.accept().await;
        println!("[Server] Accepted connection from {}", client_addr);

        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).await;
        println!("[Server] Received: {}", String::from_utf8_lossy(&buf[..n]));
        
        let response = b"Hello from JaviRust Runtime!";
        stream.write(response).await;
        println!("[Server] Response sent.");
    }
}

async fn client_logic(addr: String) {
    thread::sleep(std::time::Duration::from_millis(500));
    let addr: SocketAddr = addr.parse().unwrap();
    
    let stream = TcpStream::connect(addr).unwrap();
    let mut async_stream = AsyncTcpStream::new(stream);
    
    println!("[Client] Connected to {}", addr);
    
    async_stream.write(b"Hello Server! I am a custom runtime.").await;
    
    let mut buf = [0u8; 1024];
    let n = async_stream.read(&mut buf).await;
    println!("[Client] Received from server: {}", String::from_utf8_lossy(&buf[..n]));
}

fn main() {
    get_reactor();

    let (tx, rx) = unbounded();
    let executor = Executor { ready_queue: rx };

    let addr = "127.0.0.1:9999".to_string();

    // async fn 自动实现 Future trait，但是不能直接调用，所以我们在 main 中使用 spawn 来启动它们.
    spawn("Server", server_logic(addr.clone()), &tx);
    spawn("Client", client_logic(addr), &tx);

    println!("--- Executor Starting ---");
    executor.run();
}
