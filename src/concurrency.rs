//! 并发编程示例
//!
//! 展示Rust的并发特性：线程、通道、异步编程等

use std::env::args;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{string, thread};

/// 基础线程创建
pub fn basic_threads() {
    println!("  === 基础线程 ===");

    // 创建线程
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程继续执行
    for i in 1..=3 {
        println!("  主线程: {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    // 等待子线程完成
    handle.join().unwrap();
    println!("  子线程已完成");
}

/// 线程间数据共享
pub fn thread_shared_data() {
    println!("  === 线程间数据共享 ===");

    // 使用Arc和Mutex共享可变数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("  线程计数: {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  最终计数: {}", *counter.lock().unwrap());
}

/// 通道（Channel）通信
pub fn channel_communication() {
    println!("  === 通道通信 ===");

    // 创建通道
    let (tx, rx) = mpsc::channel();

    // 发送线程
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let messages = vec![
            String::from("你好"),
            String::from("来自"),
            String::from("子线程"),
        ];
        for msg in messages {
            tx_clone.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程发送
    thread::spawn(move || {
        let messages = vec![String::from("更多消息"), String::from("从另一个线程")];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // 接收消息
    for received in rx {
        println!("  收到: {}", received);
    }
}

/// 多生产者单消费者模式
pub fn mpsc_example() {
    println!("  === 多生产者单消费者 ===");

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    // 多个生产者
    for i in 0..3 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let msg = format!("生产者{}-消息{}", i, j);
                match tx_clone.send(msg) {
                    Ok(_) => println!("  生产者{} 发送了消息{}", i, j),
                    Err(e) => println!("  生产者{} 发送失败: {}", i, e),
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    // 关闭多余的发送端
    drop(tx);

    // 消费者
    let consumer_handle = thread::spawn(move || {
        let mut count = 0;
        for received in rx {
            println!("  消费: {}", received);
            count += 1;
        }
        println!("  总共消费了 {} 条消息", count);
    });

    // 等待所有生产者完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 等待消费者完成
    consumer_handle.join().expect("消费者线程 panicked");
}

/// 读写锁（RwLock）
pub fn rwlock_example() {
    println!("  === 读写锁示例 ===");

    use std::sync::RwLock;

    let lock = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 多个读取者
    for i in 0..5 {
        let lock_clone = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let read_guard = lock_clone.read().unwrap();
            println!("  读取者{} 读取到: {}", i, *read_guard);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    // 单个写入者
    let lock_clone = Arc::clone(&lock);
    let write_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let mut write_guard = lock_clone.write().unwrap();
        *write_guard = 42;
        println!("  写入者写入: {}", *write_guard);
    });

    for handle in handles {
        handle.join().unwrap();
    }
    write_handle.join().unwrap();

    println!("  最终值: {}", *lock.read().unwrap());
}

/// 条件变量
pub fn condition_variable_example() {
    println!("  === 条件变量示例 ===");

    use std::sync::Condvar;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    // 等待线程
    let wait_handle = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        while !*started {
            println!("  等待线程正在等待...");
            started = cvar.wait(started).unwrap();
            println!("  等待线程notified!");
        }
        println!("  等待线程被唤醒");
    });

    // 通知线程
    thread::sleep(Duration::from_millis(100));
    {
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("  已通知等待线程");
    }

    wait_handle.join().unwrap();
}

/// 原子操作
pub fn atomic_operations() {
    println!("  === 原子操作 ===");

    use std::sync::atomic::{AtomicU32, Ordering};

    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  原子计数: {}", counter.load(Ordering::SeqCst));
}

/// 异步编程基础（需要tokio依赖）
#[allow(dead_code)]
pub async fn async_basics() {
    println!("  === 异步编程基础 ===");
    println!("  （需要tokio依赖，当前未启用）");
}

/// 异步通道（需要tokio依赖）
#[allow(dead_code)]
pub async fn async_channels() {
    println!("  === 异步通道 ===");
    println!("  （需要tokio依赖，当前未启用）");
}

/// 异步任务超时（需要tokio依赖）
#[allow(dead_code)]
pub async fn async_timeout() {
    println!("  === 异步任务超时 ===");
    println!("  （需要tokio依赖，当前未启用）");
}

/// 异步流处理（需要futures依赖）
#[allow(dead_code)]
pub async fn async_stream_processing() {
    println!("  === 异步流处理 ===");
    println!("  （需要futures依赖，当前未启用）");
}

/// 并发模式：工作池
pub fn worker_pool_pattern() {
    println!("  === 工作池模式 ===");

    use std::sync::mpsc::{channel, Sender};
    use std::sync::Arc;

    type Job = Box<dyn FnOnce() + Send + 'static>;

    struct Worker {
        id: usize,
        #[allow(dead_code)]
        handle: thread::JoinHandle<()>,
    }

    struct ThreadPool {
        workers: Vec<Worker>,
        sender: Sender<Job>,
    }

    impl ThreadPool {
        fn new(size: usize) -> ThreadPool {
            let (sender, receiver) = channel::<Job>();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                let receiver_clone = Arc::clone(&receiver);
                let handle = thread::spawn(move || loop {
                    let job = receiver_clone.lock().unwrap().recv();
                    match job {
                        Ok(job) => {
                            println!("  工人{} 开始工作", id);
                            job();
                            println!("  工人{} 完成工作", id);
                        }
                        Err(_) => break,
                    }
                });
                workers.push(Worker { id, handle });
            }

            ThreadPool { workers, sender }
        }

        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            for worker in &mut self.workers {
                println!("  关闭工人{}", worker.id);
            }
        }
    }

    let pool = ThreadPool::new(3);

    for i in 0..5 {
        let param = format!("任务{}", i);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(100));
            println!("    任务{} 执行完毕 {}", i, param);
        });
    }

    // 等待所有任务完成
    thread::sleep(Duration::from_millis(600));
}

/// 运行所有并发示例
pub fn run_examples() {
    basic_threads();
    thread_shared_data();
    channel_communication();
    mpsc_example();
    rwlock_example();
    condition_variable_example();
    atomic_operations();
    worker_pool_pattern();
}

/// 运行异步示例（需要tokio运行时）
#[allow(dead_code)]
pub async fn run_async_examples() {
    async_basics().await;
    async_channels().await;
    async_timeout().await;
    async_stream_processing().await;
}
