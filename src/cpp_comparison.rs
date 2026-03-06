//! C++与Rust对比示例
//!
//! 针对有C++经验的开发者，展示Rust如何解决C++中的常见问题

use std::collections::HashMap;

/// 智能指针对比
pub fn smart_pointers_comparison() {
    println!("  === 智能指针对比 ===");

    // C++: std::unique_ptr<T>
    // Rust: Box<T> (独占所有权)
    {
        let boxed_value = Box::new(42);
        println!("  Box: {}", boxed_value);
        // 离开作用域时自动释放
    }

    // C++: std::shared_ptr<T>
    // Rust: Rc<T> (引用计数，不可变)
    use std::rc::Rc;
    {
        let rc1 = Rc::new(String::from("shared"));
        let rc2 = Rc::clone(&rc1);
        println!("  Rc引用计数: {}", Rc::strong_count(&rc1));
        println!("  rc1: {}, rc2: {}", rc1, rc2);
    }

    // C++: std::shared_ptr<T> + std::atomic
    // Rust: Arc<T> (原子引用计数，线程安全)
    use std::sync::Arc;
    {
        let arc1 = Arc::new(String::from("thread-safe"));
        let arc2 = Arc::clone(&arc1);
        println!("  Arc引用计数: {}", Arc::strong_count(&arc1));
        println!("  arc1: {}, arc2: {}", arc1, arc2);
    }

    // C++: std::unique_ptr<T> with mutable
    // Rust: Rc<RefCell<T>> 或 Arc<Mutex<T>>
    use std::cell::RefCell;
    {
        let rc_refcell = Rc::new(RefCell::new(String::from("mutable")));
        let rc_refcell_clone = Rc::clone(&rc_refcell);
        *rc_refcell_clone.borrow_mut() = String::from("modified");
        println!("  Rc<RefCell>: {}", rc_refcell.borrow());
    }
}

/// RAII对比
pub fn raii_comparison() {
    println!("  === RAII对比 ===");

    // C++: 析构函数自动调用
    // Rust: Drop trait自动调用
    struct FileHandler {
        name: String,
    }

    impl Drop for FileHandler {
        fn drop(&mut self) {
            println!("    关闭文件: {}", self.name);
        }
    }

    {
        let _file = FileHandler {
            name: String::from("example.txt"),
        };
        println!("  文件已打开");
        // 离开作用域时自动调用drop
    }
    println!("  文件已自动关闭");
}

/// 模式匹配 vs switch
pub fn pattern_matching_comparison() {
    println!("  === 模式匹配对比 ===");

    // C++: switch语句
    // Rust: match表达式（更强大）
    #[allow(dead_code)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;
    let description = match dir {
        Direction::North => "向上",
        Direction::South => "向下",
        Direction::East => "向右",
        Direction::West => "向左",
    };
    println!("  方向: {}", description);

    // 解构复杂类型
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 3, y: 7 };
    match point {
        Point { x: 0, y } => println!("  在Y轴上，y={}", y),
        Point { x, y: 0 } => println!("  在X轴上，x={}", x),
        Point { x, y } => println!("  在象限内: ({}, {})", x, y),
    }

    // 模式守卫
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("  两个数相等"),
        (x, y) if x + y == 0 => println!("  两数互为相反数"),
        (x, y) => println!("  普通情况: ({}, {})", x, y),
    }
}

/// 错误处理对比
pub fn error_handling_comparison() {
    println!("  === 错误处理对比 ===");

    // C++: 异常 vs 错误码
    // Rust: Result<T, E> 类型系统保证

    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse().map_err(|e| format!("解析失败: {}", e))
    }

    // 传统方式（类似错误码）
    match parse_number("123") {
        Ok(n) => println!("  解析成功: {}", n),
        Err(e) => println!("  错误: {}", e),
    }

    // 使用?操作符（类似异常传播）
    fn process_input(input: &str) -> Result<i32, String> {
        let num = parse_number(input)?; // 如果失败，直接返回Err
        Ok(num * 2)
    }

    match process_input("456") {
        Ok(result) => println!("  处理结果: {}", result),
        Err(e) => println!("  处理错误: {}", e),
    }

    // Option<T> 对比 nullptr/optional
    fn find_user(id: u32) -> Option<String> {
        let users: HashMap<u32, String> = [(1, "Alice".to_string()), (2, "Bob".to_string())].into();
        users.get(&id).cloned()
    }

    match find_user(1) {
        Some(name) => println!("  找到用户: {}", name),
        None => println!("  用户不存在"),
    }
}

/// 生命周期与内存安全
pub fn memory_safety_comparison() {
    println!("  === 内存安全对比 ===");

    // C++: 手动管理内存，容易出现悬垂指针
    // Rust: 编译器保证无悬垂指针

    // 这个函数在C++中可能返回悬垂指针
    // 在Rust中编译器会拒绝
    // fn dangling_reference() -> &String {
    //     let s = String::from("hello");
    //     &s // 错误：s将在函数结束时被释放
    // }

    // 正确的方式：返回所有权
    fn create_string() -> String {
        String::from("hello")
    }

    let s = create_string();
    println!("  创建的字符串: {}", s);

    // 借用检查器防止数据竞争
    let mut data = vec![1, 2, 3];
    
    // 不可变借用
    let immutable_ref = &data;
    println!("  不可变借用: {:?}", immutable_ref);
    
    // 可变借用（必须在不可变借用结束后）
    let mutable_ref = &mut data;
    mutable_ref.push(4);
    println!("  可变借用后: {:?}", mutable_ref);
}

/// 并发安全对比
pub fn concurrency_safety_comparison() {
    println!("  === 并发安全对比 ===");

    use std::sync::{Arc, Mutex};
    use std::thread;

    // C++: 需要手动使用mutex保护共享数据
    // Rust: 类型系统防止数据竞争

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  最终计数: {}", *counter.lock().unwrap());
}

/// 函数式编程特性
pub fn functional_programming_comparison() {
    println!("  === 函数式编程特性 ===");

    // C++: lambda表达式
    // Rust: 闭包（更灵活）

    let numbers = vec![1, 2, 3, 4, 5];

    // 过滤
    let even_numbers: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("  偶数: {:?}", even_numbers);

    // 映射
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("  翻倍: {:?}", doubled);

    // 折叠
    let sum: i32 = numbers.iter().sum();
    println!("  求和: {}", sum);

    // 闭包捕获环境
    let multiplier = 3;
    let multiplied: Vec<_> = numbers.iter().map(|x| x * multiplier).collect();
    println!("  乘以{}: {:?}", multiplier, multiplied);

    // 迭代器链
    let result: Vec<_> = numbers
        .iter()
        .filter(|&&x| x > 2)
        .map(|x| x * x)
        .collect();
    println!("  大于2的数的平方: {:?}", result);
}

/// 类型系统对比
pub fn type_system_comparison() {
    println!("  === 类型系统对比 ===");

    // C++: 模板元编程（复杂）
    // Rust: Trait系统（更清晰）

    // Trait对象（类似虚函数）
    trait Drawable {
        fn draw(&self);
    }

    struct Circle;
    struct Square;

    impl Drawable for Circle {
        fn draw(&self) {
            println!("    画圆形");
        }
    }

    impl Drawable for Square {
        fn draw(&self) {
            println!("    画方形");
        }
    }

    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
        Box::new(Circle),
    ];

    for shape in shapes {
        shape.draw();
    }

    // 关联类型（类似模板参数）
    trait Container {
        type Item;
        fn get(&self) -> &Self::Item;
    }

    struct IntContainer(i32);
    impl Container for IntContainer {
        type Item = i32;
        fn get(&self) -> &i32 {
            &self.0
        }
    }

    let container = IntContainer(42);
    println!("  容器值: {}", container.get());
}

/// 编译时计算
pub fn compile_time_comparison() {
    println!("  === 编译时计算 ===");

    // C++: constexpr
    // Rust: const fn 和宏

    const fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    const FIB_10: u32 = fibonacci(10);
    println!("  斐波那契数列第10项: {}", FIB_10);

    // 宏（编译时代码生成）
    macro_rules! create_vector {
        ($($x:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let vec = create_vector![1, 2, 3, 4, 5];
    println!("  宏创建的向量: {:?}", vec);
}

/// 运行所有对比示例
pub fn run_examples() {
    smart_pointers_comparison();
    raii_comparison();
    pattern_matching_comparison();
    error_handling_comparison();
    memory_safety_comparison();
    concurrency_safety_comparison();
    functional_programming_comparison();
    type_system_comparison();
    compile_time_comparison();
}