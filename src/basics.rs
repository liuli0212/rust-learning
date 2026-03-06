//! 基础语法示例
//!
//! 涵盖Rust的核心概念：变量、类型、所有权、借用、生命周期等

use core::num;


/// 变量和可变性
pub fn variables_and_mutability() {
    println!("  === 变量和可变性 ===");

    // 不可变变量（默认）
    let x = 5;
    let _y = 10;
    println!("  不可变变量 x = {}", x);
    // x = 6; // 编译错误：cannot assign twice to immutable variable

    // 可变变量
    let mut y = 10;
    println!("  可变变量 y = {}", y);
    y = 20;
    println!("  修改后 y = {}", y);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("  常量 MAX_POINTS = {}", MAX_POINTS);

    // 阴影（Shadowing）
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("  阴影后的 z = {}", z);
}

/// 所有权（Ownership）演示
pub fn ownership_demo() {
    println!("  === 所有权演示 ===");

    // 基本类型（栈分配） - 实现Copy trait
    let s1 = String::from("hello");
    let s2 = s1; // 所有权转移
    println!("  s2 = {}", s2);
    // println!("  s1 = {}", s1); // 编译错误：s1已失效

    // 克隆（深拷贝）
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("  s3 = {}, s4 = {}", s3, s4);

    // 函数调用中的所有权转移
    let s5 = String::from("ownership");
    let s6 = takes_ownership(s5);
    println!("  函数返回: {}", s6);
    // println!("  s5 = {}", s5); // 编译错误：s5已转移
}

fn takes_ownership(s: String) -> String {
    println!("    函数内: {}", s);
    s // 返回所有权
}

/// 借用（Borrowing）演示
pub fn borrowing_demo() {
    println!("  === 借用演示 ===");

    let s1 = String::from("borrowing");

    // 不可变借用
    let len = calculate_length(&s1);
    println!("  字符串: {}, 长度: {}", s1, len);

    // 可变借用
    let mut s2 = String::from("mutable");
    change_string(&mut s2);
    println!("  修改后: {}", s2);

    // 注意：同一作用域内不能同时有多个可变借用
    // let r1 = &mut s2;
    // let r2 = &mut s2; // 编译错误

    let mut ss = String::from("hello");
    calculate_length_mut(&mut ss);
    tt(ss);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_length_mut(s: &mut String) -> usize {
    s.push_str(" borrow");
    s.len()
}

fn tt(s: String) -> usize{
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(" borrow");
}

/// 生命周期基础
pub fn lifetime_basics() {
    println!("  === 生命周期基础 ===");

    let string1 = String::from("long string");
    let string2 = String::from("short");

    let result = longest(&string1, &string2);
    println!("  最长的字符串: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 结构体和枚举
pub fn structs_and_enums() {
    println!("  === 结构体和枚举 ===");

    // 结构体
    #[derive(Debug)]
    #[allow(dead_code)]
    struct User {
        username: String,
        email: String,
        active: bool,
    }

    let user1 = User {
        username: String::from("rustacean"),
        email: String::from("rust@example.com"),
        active: true,
    };

    println!("  用户: {:?}", user1);

    // 枚举
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        #[allow(dead_code)]
        ChangeColor(i32, i32, i32),
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4: Message = Message::ChangeColor(255, 0, 0);

    println!("  消息1: {:?}", msg1);
    println!("  消息2: {:?}", msg2);
    println!("  消息3: {:?}", msg3);
    println!("  消息4: {:?}", msg4);

    // 模式匹配
    match msg2 {
        Message::Quit => println!("    退出"),
        Message::Move { x, y } => println!("    移动到 ({}, {})", x, y),
        Message::Write(text) => println!("    写入: {}", text),
        Message::ChangeColor(r, g, b) => println!("    颜色: ({}, {}, {})", r, g, b),
    }
}

/// Option类型详解
pub fn option_examples() {
    println!("  === Option类型详解 ===");

    // 1. 创建Option
    let some_number: Option<i32> = Some(42);
    let some_string: Option<&str> = Some("hello");
    let none_value: Option<i32> = None;

    println!("  some_number: {:?}", some_number);
    println!("  some_string: {:?}", some_string);
    println!("  none_value: {:?}", none_value);

    // 2. 模式匹配
    match some_number {
        Some(num) => println!("  值是: {}", num),
        None => println!("  没有值"),
    }

    // 3. unwrap - 如果是None会panic
    let value = some_number.unwrap();
    println!("  unwrap结果: {}", value);
    // let panic_value = none_value.unwrap(); // 会panic!

    // 4. unwrap_or - 提供默认值
    let result1 = some_number.unwrap_or(0);
    let result2 = none_value.unwrap_or(0);
    println!("  unwrap_or: {} 和 {}", result1, result2);

    // 5. unwrap_or_else - 惰性计算默认值
    let result3 = none_value.unwrap_or_else(|| {
        println!("    计算默认值...");
        999
    });
    println!("  unwrap_or_else结果: {}", result3);

    // 6. map - 转换Option中的值
    let doubled = some_number.map(|x| x * 2);
    println!("  map结果: {:?}", doubled);

    println!("  None in print 结果: {:?}", none_value);

    // 7. and_then - 链式操作
    let result4 = some_number.and_then(|x| {
        if x > 10 {
            Some(x * 2)
        } else {
            None
        }
    });
    println!("  and_then结果: {:?}", result4);

    // 8. filter - 过滤Option
    let filtered = some_number.filter(|&x| x > 50);
    println!("  filter结果: {:?}", filtered);

    // 9. is_some / is_none
    println!("  is_some: {}", some_number.is_some());
    println!("  is_none: {}", none_value.is_none());

    // 10. as_ref / as_mut
    let ref_some = some_number.as_ref();
    println!("  as_ref结果: {:?}", ref_some);

    // 11. take - 取出值并置为None
    let mut mutable_option = Some(100);
    let taken = mutable_option.take();
    println!("  take结果: {:?}, 原Option: {:?}", taken, mutable_option);

    // 12. 实际应用：从数组中查找
    let numbers = [1, 3, 5, 8, 9, 12];
    let first_even = numbers.iter().find(|&&x| x % 2 == 0).copied();
    match first_even {
        Some(even) => println!("  第一个偶数: {}", even),
        None => println!("  没有偶数"),
    }

    // 13. Option的迭代器操作
    let optional_numbers: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(4)];
    let sum: i32 = optional_numbers.iter()
        .filter_map(|&x| x)  // 过滤掉None
        .sum();
    println!("  Option列表求和: {}", sum);

    // 14. ? 操作符在函数中使用
    fn get_first_even(numbers: &[i32]) -> Option<i32> {
        numbers.iter().find(|&&x| x % 2 == 0).copied()
    }

    fn process_numbers(numbers: &[i32]) -> Option<i32> {
        let first = get_first_even(numbers)?;  // 如果是None，提前返回None
        Some(first * 2)
    }

    let result = process_numbers(&[1, 3, 5, 8]);
    println!("  process_numbers结果: {:?}", result);

    // 15. Option与Result的转换
    let option_to_result: Result<i32, &str> = some_number.ok_or("值不存在");
    println!("  Option转Result: {:?}", option_to_result);
}

/// 错误处理
pub fn error_handling() {
    println!("  === 错误处理 ===");

    // Result类型
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10, 2) {
        Ok(result) => println!("  10 / 2 = {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("  10 / 0 = {}", result),
        Err(e) => println!("  错误: {}", e),
    }
}

/// 泛型基础
pub fn generics_basics() {
    println!("  === 泛型基础 ===");

    // 泛型函数
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = [34, 50, 25, 100, 65];
    println!("  数字列表中最大的: {}", largest(&numbers));

    let chars = ['y', 'm', 'a', 'q'];
    println!("  字符列表中最大的: {}", largest(&chars));

    // 泛型结构体
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("  整数点: {:?}", integer_point);
    println!("  浮点点: {:?}", float_point);
}

/// Trait基础
pub fn trait_basics() {
    println!("  === Trait基础 ===");

    // 定义trait
    trait Summary {
        fn summarize(&self) -> String;
    }

    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    struct Tweet {
        username: String,
        content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let article = NewsArticle {
        headline: String::from("Rust 1.75发布"),
        location: String::from("北京"),
        author: String::from("官方团队"),
    };

    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("学习Rust很有趣！"),
    };

    println!("  文章摘要: {}", article.summarize());
    println!("  推文摘要: {}", tweet.summarize());

    // trait作为参数
    fn notify(item: &impl Summary) {
        println!("    通知: {}", item.summarize());
    }

    notify(&article);
    notify(&tweet);
}

/// Box用法示例
pub fn box_examples() {
    println!("  === Box用法示例 ===");

    // 1. 基本Box使用 - 在堆上分配数据
    let boxed_number = Box::new(42);
    println!("  Box中的数字: {}", boxed_number);
    println!("  Box中的数字（解引用）: {}", *boxed_number);

    // 2. Box用于递归类型（编译时未知大小的类型）
    // 定义一个链表节点
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // 创建链表: 1 -> 2 -> 3 -> Nil
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("  链表: {:?}", list);

    // 3. Box用于在堆上存储大型结构体（避免栈溢出）
    #[derive(Debug)]
    struct LargeData {
        data: [u8; 1024 * 1024], // 1MB的数据
    }

    // 如果直接在栈上创建可能会导致栈溢出
    let boxed_large = Box::new(LargeData { data: [0; 1024 * 1024] });
    println!("  大型结构体Box大小: {} bytes", std::mem::size_of_val(&boxed_large));

    // 4. Box用于多态（通过trait对象）
    trait Animal {
        fn speak(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn speak(&self) {
            println!("    汪汪！");
        }
    }

    impl Animal for Cat {
        fn speak(&self) {
            println!("    喵喵！");
        }
    }

    // Box<dyn Animal>允许存储不同的Animal实现
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals {
        animal.speak();
    }

    // 5. Box用于函数返回（避免返回大型栈数据）
    fn create_large_array() -> Box<[u32; 1000]> {
        let mut arr = [0; 1000];
        for i in 0..1000 {
            arr[i] = i as u32;
        }
        Box::new(arr)
    }

    let large_array = create_large_array();
    println!("  大型数组前3个元素: {}, {}, {}", large_array[0], large_array[1], large_array[2]);

    // 6. Box用于智能指针的嵌套
    let nested_box = Box::new(Box::new(100));
    println!("  嵌套Box: {}", **nested_box);

    // 7. Box用于自引用结构（需要小心处理生命周期）
    #[derive(Debug)]
    struct SelfReferential {
        value: i32,
        // 注意：Rust中直接自引用是不安全的，这里只是演示概念
        // 实际使用中通常需要使用Pin或其他机制
    }

    let self_ref = SelfReferential { value: 42 };
    println!("  自引用结构: {:?}", self_ref);

    // 8. Box用于动态大小类型（DST）
    // Box<dyn Trait>是动态大小类型的一个例子
    trait Process {
        fn process(&self) -> String;
    }

    struct ProcessorA;
    struct ProcessorB;

    impl Process for ProcessorA {
        fn process(&self) -> String {
            String::from("ProcessorA处理")
        }
    }

    impl Process for ProcessorB {
        fn process(&self) -> String {
            String::from("ProcessorB处理")
        }
    }

    let processors: Vec<Box<dyn Process>> = vec![
        Box::new(ProcessorA),
        Box::new(ProcessorB),
    ];

    for processor in processors {
        println!("  {}", processor.process());
    }

    // 9. Box用于避免循环引用（配合Rc/Arc使用）
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<Node>>,
    }

    let node3 = Rc::new(Node { value: 3, next: None });
    let node2 = Rc::new(Node { value: 2, next: Some(Rc::clone(&node3)) });
    let node1 = Rc::new(Node { value: 1, next: Some(Rc::clone(&node2)) });

    println!("  链表节点: {:?}", node1);

    // 10. Box用于trait对象的动态分发
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
    ];

    for shape in shapes {
        shape.draw();
    }
}

/// dyn语法示例（动态分发）
pub fn dyn_examples() {
    println!("  === dyn语法示例 ===");

    // 1. dyn Trait基础 - 动态分发
    trait Animal {
        fn name(&self) -> &str;
        fn speak(&self);
    }

    struct Dog;
    struct Cat;
    struct Bird;

    impl Animal for Dog {
        fn name(&self) -> &str {
            "狗"
        }
        fn speak(&self) {
            println!("    汪汪！");
        }
    }

    impl Animal for Cat {
        fn name(&self) -> &str {
            "猫"
        }
        fn speak(&self) {
            println!("    喵喵！");
        }
    }

    impl Animal for Bird {
        fn name(&self) -> &str {
            "鸟"
        }
        fn speak(&self) {
            println!("    叽叽喳喳！");
        }
    }

    // 使用dyn Trait创建trait对象
    // Box<dyn Animal>允许在运行时决定调用哪个实现
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Bird),
    ];

    println!("  动态分发示例:");
    for animal in animals {
        println!("    {}:", animal.name());
        animal.speak();
    }

    // 2. dyn Trait作为函数参数
    fn make_sound(animal: &dyn Animal) {
        println!("  {}在叫:", animal.name());
        animal.speak();
    }

    let dog = Dog;
    let cat = Cat;
    make_sound(&dog);
    make_sound(&cat);

    // 3. dyn Trait作为返回值
    fn get_animal(animal_type: &str) -> Box<dyn Animal> {
        match animal_type {
            "dog" => Box::new(Dog),
            "cat" => Box::new(Cat),
            "bird" => Box::new(Bird),
            _ => Box::new(Dog), // 默认返回Dog
        }
    }

    let animal = get_animal("cat");
    println!("  获取的动物: {}", animal.name());
    animal.speak();

    // 4. dyn Trait与泛型的对比
    // 静态分发（泛型）- 编译时确定类型
    fn static_dispatch<T: Animal>(animal: &T) {
        println!("  静态分发 - {}:", animal.name());
        animal.speak();
    }

    // 动态分发（dyn Trait）- 运行时确定类型
    fn dynamic_dispatch(animal: &dyn Animal) {
        println!("  动态分发 - {}:", animal.name());
        animal.speak();
    }

    let my_dog = Dog;
    static_dispatch(&my_dog);
    dynamic_dispatch(&my_dog);

    // 5. dyn Trait与Vec结合
    trait Shape {
        fn area(&self) -> f64;
        fn name(&self) -> &str;
    }

    struct CircleShape {
        radius: f64,
    }

    struct RectangleShape {
        width: f64,
        height: f64,
    }

    impl Shape for CircleShape {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
        fn name(&self) -> &str {
            "圆形"
        }
    }

    impl Shape for RectangleShape {
        fn area(&self) -> f64 {
            self.width * self.height
        }
        fn name(&self) -> &str {
            "矩形"
        }
    }

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(CircleShape { radius: 5.0 }),
        Box::new(RectangleShape { width: 4.0, height: 6.0 }),
    ];

    println!("  形状面积计算:");
    for shape in shapes {
        println!("    {}: 面积 = {:.2}", shape.name(), shape.area());
    }

    // 6. dyn Trait与Option结合
    fn find_shape(shape_type: &str) -> Option<Box<dyn Shape>> {
        match shape_type {
            "circle" => Some(Box::new(CircleShape { radius: 3.0 })),
            "rectangle" => Some(Box::new(RectangleShape { width: 2.0, height: 4.0 })),
            _ => None,
        }
    }

    if let Some(shape) = find_shape("circle") {
        println!("  找到形状: {}, 面积: {:.2}", shape.name(), shape.area());
    }

    // 7. dyn Trait与Result结合
    fn create_shape(shape_type: &str) -> Result<Box<dyn Shape>, String> {
        match shape_type {
            "circle" => Ok(Box::new(CircleShape { radius: 5.0 })),
            "rectangle" => Ok(Box::new(RectangleShape { width: 4.0, height: 6.0 })),
            _ => Err(format!("未知的形状类型: {}", shape_type)),
        }
    }

    match create_shape("triangle") {
        Ok(shape) => println!("  创建成功: {}, 面积: {:.2}", shape.name(), shape.area()),
        Err(e) => println!("  创建失败: {}", e),
    }

    // 8. dyn Trait的大小和内存布局
    // dyn Trait是动态大小类型（DST），不能直接存储，必须通过指针
    // Box<dyn Animal>的大小是固定的（指针大小）
    println!("  Box<dyn Animal>的大小: {} bytes", std::mem::size_of::<Box<dyn Animal>>());

    // 9. dyn Trait与Send/Sync trait
    // dyn Send和dyn Sync用于多线程环境
    trait ThreadSafe: Send + Sync {
        fn process(&self);
    }

    struct SafeData;

    impl ThreadSafe for SafeData {
        fn process(&self) {
            println!("    线程安全处理");
        }
    }

    let safe: Box<dyn ThreadSafe> = Box::new(SafeData);
    safe.process();

    // 10. dyn Trait的生命周期
    trait Processor {
        fn process(&self, data: &str) -> String;
    }

    struct UpperCaseProcessor;
    struct LowerCaseProcessor;

    impl Processor for UpperCaseProcessor {
        fn process(&self, data: &str) -> String {
            data.to_uppercase()
        }
    }

    impl Processor for LowerCaseProcessor {
        fn process(&self, data: &str) -> String {
            data.to_lowercase()
        }
    }

    fn process_with_dyn(processor: &dyn Processor, data: &str) -> String {
        processor.process(data)
    }

    let upper = UpperCaseProcessor;
    let lower = LowerCaseProcessor;

    let text = "Hello World";
    println!("  原始文本: {}", text);
    println!("  大写处理: {}", process_with_dyn(&upper, text));
    println!("  小写处理: {}", process_with_dyn(&lower, text));

    // 11. dyn Trait的性能考虑
    // 静态分发（泛型）在编译时确定类型，没有运行时开销
    // 动态分发（dyn Trait）在运行时查找虚表（vtable），有少量开销
    // 但提供了更大的灵活性

    // 12. dyn Trait的实际应用场景
    // 插件系统、回调函数、异构集合等
    trait Plugin {
        fn name(&self) -> &str;
        fn execute(&self);
    }

    struct LoggingPlugin;
    struct MetricsPlugin;

    impl Plugin for LoggingPlugin {
        fn name(&self) -> &str {
            "日志插件"
        }
        fn execute(&self) {
            println!("    执行日志记录");
        }
    }

    impl Plugin for MetricsPlugin {
        fn name(&self) -> &str {
            "指标插件"
        }
        fn execute(&self) {
            println!("    收集性能指标");
        }
    }

    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(LoggingPlugin),
        Box::new(MetricsPlugin),
    ];

    println!("  插件系统:");
    for plugin in plugins {
        println!("    {}:", plugin.name());
        plugin.execute();
    }
}

/// 运行所有基础示例
pub fn run_examples() {
    variables_and_mutability();
    ownership_demo();
    borrowing_demo();
    lifetime_basics();
    structs_and_enums();
    option_examples();
    error_handling();
    generics_basics();
    trait_basics();
    box_examples();
    dyn_examples();
}