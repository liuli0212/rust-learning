macro_rules! count_exprs {
    () => (0);
    ($head:tt $($tail:tt)*) => (1 + count_exprs!($($tail)*));
}


macro_rules! select {
    ($doc:item) => {
        println!("Matched! {:?}", stringify!($doc));
        println!("declaring func");
        // 这里$doc 是一个 item，可以是函数、模块、结构体等，如果是代码，那就相当于直接把它展开了。
        $doc
    }
}

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// 一个简单的声明式宏，演示 select! 的本质
macro_rules! my_select {
    // 语法：my_select! { name = future => { block }, ... ,?}, $(,)? 允许可选的逗号结尾
    /*
        在 Rust 宏中，如果你在同一个 $( ... )+ 或 $( ... )* 匹配块中定义了多个变量（比如你的
        $name、$fut 和 $handler），那么在展开时，它们会“成对地（同步地）”被使用。
        你可以把它想象成在处理一个结构体列表或者 Tuple 列表：
        1. 匹配时：宏引擎发现你传入了两组：
        - 第一组：$name = val_a, $fut = task_a, $handler = block_a
        - 第二组：$name = val_b, $fut = task_b, $handler = block_b

        2. 展开时：当你写 $( ... )* 时，宏引擎会同步地取出每一组：
      - 第一次循环：它会把里面的 $name 替换成 val_a，$fut 替换成 task_a，$handler
        替换成 block_a。
      - 第二次循环：它会把里面的 $name 替换成 val_b，$fut 替换成 task_b，$handler
        替换成 block_b。
     */
    ( $( $name:ident = $fut:expr => $handler:expr),+ $(,)? ) => {
        {
            println!("Entering my_select! macro with {} branches", count_exprs!($($name)*));
            println!("Futures names to poll: {}", stringify!($($name.len()),+));
            println!("$fut expressions: {}", stringify!($($fut),+));
            println!("$handler expressions: {}", stringify!($($handler),+));

            // 我们利用 poll_fn 来获取当前的 Context (cx)
            std::future::poll_fn(|cx| {
                $(
                    // 1. 尝试对每一个 Future 进行轮询
                    // 这里假设 $fut 是已经 Pin 过的，或者可以安全 poll 的
                    //match Pin::new(&mut $fut).poll(cx) {
                    match $fut.as_mut().poll(cx) {
                        Poll::Ready(val) => {
                            // 2. 如果某个 Future 好了，直接返回 Ready 包装的 handler 结果
                            let $name = val;
                            // 打印val的值  
                            println!("READY! '{}' is '{}'", stringify!($name), $name);
                            return Poll::Ready($handler);
                        }
                        Poll::Pending => {
                            // 3. 如果没好，继续下一个分支
                        }
                    }
                )*

                // 4. 如果这一轮所有的 Future 都 Pending，则通知 Runtime 还没好
                Poll::Pending
            }).await
        }
    };
}

use rust_learning::Builder;

/// 一个使用我们编写的过程宏派生的结构体
#[derive(Builder, Debug, Clone)]
struct User {
    name: String,
    age: u32,
    email: String,
}

async fn test_self_with_macro() {
    println!("=== 过程宏演示 (Builder 派生) ===");

    // 使用自动生成的 builder() 方法和 UserBuilder 结构体
    let user_res = User::builder()
        .name("Alice".to_string())
        .age(30)
        .email("alice@example.com".to_string())
        .build();

    match user_res {
        Ok(user) => println!("成功构造用户: {:?}", user),
        Err(e) => println!("构造失败: {}", e),
    }

    // 演示缺少字段的情况
    println!("\n演示缺少字段:");
    let incomplete_user = User::builder()
        .name("Bob".to_string())
        // 故意漏掉 age 和 email
        .build();

    if let Err(e) = incomplete_user {
        println!("预期的错误: {}", e);
    }

    println!("\n=== 声明式宏演示 (my_select!) ===");
    // 模拟两个异步任务
    let task_a = async {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        "任务 A 完成"
    };

    let task_b = async {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        "任务 B 完成"
    };

    // 在 Rust 中，使用 tokio::pin! 或 std::pin::pin! 将 Future 固定在栈上
    tokio::pin!(task_a);
    tokio::pin!(task_b);

    println!("正在等待任务...");

    // 使用我们自己定义的宏
    let result = my_select! {
        val_a = task_a => { 
            println!("分支 A 的 Future 已经完成了，准备处理结果...");
            format!("分支 A 赢了: {}", val_a) 
        },
        val_b = task_b => {
            println!("分支 B 的 Future 已经完成了，准备处理结果...");
            format!("分支 B 赢了: {}", val_b) 
        },
    };

    println!("{}", result);
}

// ==========================================
// 技巧 1: 一进制计数器查找表
// ==========================================
macro_rules! count_to_num {
    () => { 0 };
    (_) => { 1 };
    (_ _) => { 2 };
    (_ _ _) => { 3 };
}

// ==========================================
// 技巧 2: 元组占位符解构
// ==========================================
macro_rules! get_from_tuple {
    ($tuple:expr, ($($skip:tt)*)) => {{
        let ($($skip,)* val, ..) = $tuple;
        val
    }};
}

// ==========================================
// 技巧 3: 主宏 mini_select!
// ==========================================
macro_rules! mini_select {
    // 【入口】
    ( $($name:ident = $val:expr => $handler:expr),+ $(,)? ) => {
        // 使用 [ ] 包裹已处理分支，避免与 ; 产生歧义
        mini_select!(@ { [ ] } ($($name = $val => $handler),+) () )
    };

    // 【递归步】TT Munching
    ( 
        @ { [ $($processed:tt)* ] } 
        ($name:ident = $val:expr => $handler:expr $(, $($rest:tt)* )? )
        ($($count:tt)*)
    ) => {
        mini_select!(
            @ { [ $($processed)* ( ($($count)*) $name = $val => $handler ) ] }
            ( $($($rest)*)? )
            ($($count)* _)
        )
    };

    // 【终结步】
    (
        @ { [ $( ( ($($skip:tt)*) $name:ident = $val:expr => $handler:expr ) )+ ] }
        ()
        ($($total:tt)*)
    ) => {
        {
            let num_branches = count_to_num!( $($total)* );
            println!(">>> 宏生成：发现 {} 个分支", num_branches);

            let data_tuple = ( $($val,)+ );
            let pick = rand::random::<usize>() % num_branches;
            println!(">>> 宏调度：随机选中索引 {}", pick);

            match pick {
                $(
                    count_to_num!( $($skip)* ) => {
                        let $name = get_from_tuple!(data_tuple, ($($skip)*));
                        $handler
                    }
                )*
                _ => unreachable!()
            }
        }
    };
}

fn main() {
    println!("=== Tokio Select 宏原理解析 ===");

    let result = mini_select! {
        msg = "Hello Macro" => { format!("处理字符串: {}", msg) },
        num = 42 => { format!("处理数字: {}", num) },
        flag = true => { format!("处理布尔值: {}", flag) },
    };

    println!("最终执行的分支结果: {}", result);
}
