//! 高级特性示例
//!
//! 展示Rust的高级特性：trait、泛型、宏、元编程等

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Deref, DerefMut};

/// 高级Trait系统
pub fn advanced_traits() {
    println!("  === 高级Trait系统 ===");

    // 关联类型
    trait Container {
        type Item;
        fn get(&self) -> &Self::Item;
        fn set(&mut self, item: Self::Item);
    }

    struct IntContainer {
        value: i32,
    }

    impl Container for IntContainer {
        type Item = i32;
        fn get(&self) -> &i32 {
            &self.value
        }
        fn set(&mut self, item: i32) {
            self.value = item;
        }
    }

    let mut container = IntContainer { value: 42 };
    println!("  关联类型容器: {}", container.get());
    container.set(100);
    println!("  修改后: {}", container.get());

    // 默认泛型类型参数
    trait Merge<T = Self> {
        fn merge(&self, other: T) -> Self;
    }

    impl Merge for i32 {
        fn merge(&self, other: i32) -> i32 {
            self + other
        }
    }

    let a = 10;
    let b = 20;
    println!("  合并结果: {}", a.merge(b));

    // 扩展trait（类似C++的ADL）
    trait StringExtension {
        fn to_camel_case(&self) -> String;
    }

    impl StringExtension for str {
        fn to_camel_case(&self) -> String {
            let mut result = String::new();
            let mut capitalize_next = true;
            for c in self.chars() {
                if c == '_' || c == ' ' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            }
            result
        }
    }

    println!("  驼峰命名: {}", "hello_world".to_camel_case());

    // trait对象与动态分发
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

/// 高级泛型
pub fn advanced_generics() {
    println!("  === 高级泛型 ===");

    // const泛型（编译时常量）
    fn print_array<T, const N: usize>(arr: [T; N])
    where
        T: Display,
    {
        print!("  数组: [");
        for (i, item) in arr.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", item);
        }
        println!("]");
    }

    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = ["a", "b", "c"];
    print_array(arr1);
    print_array(arr2);

    // 高阶生命周期
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("  公告: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s1 = String::from("longer");
    let s2 = String::from("short");
    let result = longest_with_an_announcement(&s1, &s2, "比较结果:");
    println!("  最长的: {}", result);

    // 类型约束的组合
    fn process_items<T>(items: &[T])
    where
        T: Display + Clone + PartialOrd,
    {
        if let Some(first) = items.first() {
            println!("  第一个元素: {}", first);
        }
        println!("  总共 {} 个元素", items.len());
    }

    let numbers = vec![1, 2, 3, 4, 5];
    process_items(&numbers);
}

/// 操作符重载
pub fn operator_overloading() {
    println!("  === 操作符重载 ===");

    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "({:.2}, {:.2})", self.x, self.y)
        }
    }

    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let p3 = p1 + p2;
    println!("  {} + {} = {}", p1, p2, p3);

    // 自定义加法运算
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl Add for Complex {
        type Output = Complex;

        fn add(self, other: Complex) -> Complex {
            Complex {
                real: self.real + other.real,
                imag: self.imag + other.imag,
            }
        }
    }

    impl Display for Complex {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 3.0, imag: 4.0 };
    let c3 = c1 + c2;
    println!("  复数加法: {}", c3);
}

/// 智能指针自定义
pub fn custom_smart_pointers() {
    println!("  === 自定义智能指针 ===");

    // 自定义Box
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(value: T) -> MyBox<T> {
            MyBox(value)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    let mut box1 = MyBox::new(String::from("hello"));
    println!("  MyBox内容: {}", *box1);

    // 自动解引用
    println!("  字符串长度: {}", box1.len());

    *box1 = String::from("world");
    println!("  修改后: {}", *box1);

    // 自定义Rc
    use std::rc::Rc;
    use std::cell::RefCell;

    struct MyRc<T> {
        inner: Rc<RefCell<T>>,
    }

    impl<T> MyRc<T> {
        fn new(value: T) -> MyRc<T> {
            MyRc {
                inner: Rc::new(RefCell::new(value)),
            }
        }

        fn clone(&self) -> MyRc<T> {
            MyRc {
                inner: Rc::clone(&self.inner),
            }
        }
    }

    impl<T> Deref for MyRc<T> {
        type Target = RefCell<T>;

        fn deref(&self) -> &RefCell<T> {
            &self.inner
        }
    }

    let my_rc = MyRc::new(vec![1, 2, 3]);
    let my_rc_clone = my_rc.clone();
    
    my_rc_clone.borrow_mut().push(4);
    println!("  MyRc内容: {:?}", my_rc.borrow());
}

/// 宏编程
pub fn macro_programming() {
    println!("  === 宏编程 ===");

    // 声明宏
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

    let vec1 = create_vector![1, 2, 3, 4, 5];
    println!("  宏创建的向量: {:?}", vec1);

    // 模式匹配宏
    macro_rules! match_value {
        ($value:expr, $pattern:pat => $result:expr) => {
            match $value {
                $pattern => $result,
                _ => panic!("模式不匹配"),
            }
        };
    }

    let result = match_value!(5, 5 => 5 * 2);
    println!("  宏匹配结果: {}", result);

    // 可变参数宏
    macro_rules! sum {
        ($($x:expr),*) => {
            {
                let mut total = 0;
                $(
                    total += $x;
                )*
                total
            }
        };
    }

    let total = sum!(1, 2, 3, 4, 5);
    println!("  求和结果: {}", total);

    // 类型检查宏
    macro_rules! check_type {
            ($value:expr, $type:ty) => {
                {
                    let _: $type = $value;
                    true
                }
            };
        }

    let is_i32 = check_type!(42, i32);
    println!("  是i32类型: {}", is_i32);
}

/// 过程宏（简化示例）
pub fn procedural_macros() {
    println!("  === 过程宏概念 ===");

    // 这里展示概念，实际的过程宏需要单独的crate
    // 类似C++的模板元编程，但在编译时操作AST

    println!("  过程宏允许:");
    println!("    1. 生成代码");
    println!("    2. 修改现有代码");
    println!("    3. 创建领域特定语言(DSL)");
    println!("    4. 编译时元编程");

    // 常见的派生宏示例（概念）
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    struct Example {
        x: i32,
        y: i32,
    }

    let ex = Example { x: 1, y: 2 };
    println!("  派生宏示例: {:?}", ex);
}

/// 闭包与捕获
pub fn closures_and_capture() {
    println!("  === 闭包与捕获 ===");

    // 捕获不可变引用
    let x = 5;
    let equal_to_x = |z| z == x;
    println!("  5 == 5? {}", equal_to_x(5));

    // 捕获可变引用
    let mut y = 10;
    let mut add_to_y = |z| {
        y += z;
        y
    };
    println!("  加5后: {}", add_to_y(5));

    // 移动所有权
    let s = String::from("hello");
    let take_ownership = move |s| s;
    let s2 = take_ownership(s);
    println!("  移动后的字符串: {}", s2);

    // 闭包作为参数
    fn apply<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let double = |x| x * 2;
    println!("  闭包作为参数: {}", apply(double, 21));
}

/// 迭代器高级用法
pub fn advanced_iterators() {
    println!("  === 迭代器高级用法 ===");

    use std::iter::Iterator;

    // 自定义迭代器
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Fibonacci {
        fn new() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr;
            self.curr = self.next;
            self.next = current + self.next;
            Some(current)
        }
    }

    let fib = Fibonacci::new();
    let fib_nums: Vec<_> = fib.take(10).collect();
    println!("  斐波那契数列前10项: {:?}", fib_nums);

    // 迭代器链
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<_> = numbers
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .take(3)
        .collect();
    println!("  迭代器链结果: {:?}", result);

    // 惰性求值
    let lazy_iter = (0..1000)
        .map(|x| {
            println!("  计算: {}", x);
            x * 2
        })
        .filter(|&x| x > 100);

    println!("  惰性迭代器（只计算前3个）:");
    for (i, val) in lazy_iter.take(3).enumerate() {
        println!("    {}: {}", i, val);
    }
}

/// 类型状态模式
pub fn type_state_pattern() {
    println!("  === 类型状态模式 ===");

    // 使用类型系统编码状态
    struct Draft;
    struct Published;

    struct Post<State> {
        content: String,
        _state: std::marker::PhantomData<State>,
    }

    impl Post<Draft> {
        fn new(content: String) -> Post<Draft> {
            Post {
                content,
                _state: std::marker::PhantomData,
            }
        }

        fn add_content(&mut self, text: &str) {
            self.content.push_str(text);
        }

        fn publish(self) -> Post<Published> {
            Post {
                content: self.content,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl Post<Published> {
        fn content(&self) -> &str {
            &self.content
        }
    }

    let mut draft = Post::new(String::from("我的博客"));
    draft.add_content(" - 第一部分");
    let published = draft.publish();
    println!("  已发布内容: {}", published.content());
}

/// 编译时计算
pub fn compile_time_computation() {
    println!("  === 编译时计算 ===");

    // const函数
    const fn factorial(n: u32) -> u32 {
        match n {
            0 => 1,
            _ => n * factorial(n - 1),
        }
    }

    const FACT_5: u32 = factorial(5);
    println!("  5! = {}", FACT_5);

    // const泛型数组
    fn create_array<const N: usize>() -> [i32; N] {
        let mut arr = [0; N];
        for i in 0..N {
            arr[i] = i as i32 * 2;
        }
        arr
    }

    let arr: [i32; 5] = create_array();
    println!("  const泛型数组: {:?}", arr);

    // 编译时字符串处理
    const fn str_len(s: &str) -> usize {
        s.len()
    }

    const MSG_LEN: usize = str_len("Hello, Rust!");
    println!("  字符串长度: {}", MSG_LEN);
}

/// 运行所有高级特性示例
pub fn run_examples() {
    advanced_traits();
    advanced_generics();
    operator_overloading();
    custom_smart_pointers();
    macro_programming();
    procedural_macros();
    closures_and_capture();
    advanced_iterators();
    type_state_pattern();
    compile_time_computation();
}