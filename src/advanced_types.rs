//! 高级类型系统 (Advanced Type System)
//!
//! 这个模块展示了 Rust 类型系统中最复杂但也最强大的部分，
//! 包括 GATs、HRTBs 和类型状态模式，它们是构建健壮库的基石。

/// 1. GATs (Generic Associated Types - 泛型关联类型)
/// GATs 允许在 trait 的关联类型中使用泛型（包括生命周期），
/// 这是实现诸如“借用迭代器”等高级模式的唯一方法。
pub fn gat_examples() {
    println!("  === GATs (泛型关联类型) ===");

    // 定义一个可以产生引用的集合 trait
    trait Iterable {
        // 这里的关联类型 Item 附带了一个生命周期参数 'a
        type Item<'a> where Self: 'a;
        
        // 迭代器类型也带生命周期
        type Iterator<'a>: Iterator<Item = Self::Item<'a>> where Self: 'a;

        fn iter<'a>(&'a self) -> Self::Iterator<'a>;
    }

    // 为标准的 Vec 实现 Iterable
    impl<T> Iterable for Vec<T> {
        type Item<'a> = &'a T where Self: 'a;
        type Iterator<'a> = std::slice::Iter<'a, T> where Self: 'a;

        fn iter<'a>(&'a self) -> Self::Iterator<'a> {
            self.as_slice().iter()
        }
    }

    let my_vec = vec![1, 2, 3];
    print!("    GAT 迭代结果: ");
    for item in my_vec.iter() {
        print!("{} ", item);
    }
    println!();
}

/// 2. HRTBs (Higher-Rank Trait Bounds - 高阶 Trait 约束)
/// HRTB (for<'a>) 解决的是：闭包或函数需要在*被调用时*才能确定生命周期，
/// 而不是在被定义或传递时。
pub fn hrtb_examples() {
    println!("\n  === HRTBs (高阶 Trait 约束) ===");

    // 一个接受闭包的函数，这个闭包必须能接受*任意*生命周期的 &str
    fn call_with_str<F>(closure: F) 
    where 
        // 读作："对于任意生命周期 'a，F 必须实现了 Fn(&'a str)"
        F: for<'a> Fn(&'a str) 
    {
        let local_string = String::from("Hello HRTB!");
        // 闭包在这里被调用，生命周期仅限于这个局部变量
        closure(&local_string);
    }

    // 这个闭包不绑定到任何外部的特定生命周期
    let printer = |s: &str| println!("    HRTB 闭包收到: {}", s);
    
    call_with_str(printer);
}

/// 3. Typestate Pattern (类型状态模式)
/// 利用泛型和所有权在编译期验证状态机，防止非法的状态转换。
/// 这种模式在 Rust 中被广泛使用（例如 Builder 模式）。
pub fn typestate_pattern() {
    println!("\n  === Typestate Pattern (类型状态模式) ===");

    // 状态标记类型（零大小类型，Zero Sized Types）
    // 在 C++ 中这类似于空结构体，但在 Rust 中它们完全不占用运行空间
    struct Locked;
    struct Unlocked;

    // 门结构体，带有一个状态泛型 S
    struct Door<S> {
        // PhantomData 告诉编译器：虽然 S 没在字段里用到，但它在逻辑上是属于 Door 的
        // 编译器在运行时会完全消除 marker，占用 0 内存
        _marker: std::marker::PhantomData<S>,
    }

    // --- 状态 1: 只有初始状态可以新建 ---
    impl Door<Locked> {
        fn new() -> Self {
            println!("    创建了一扇 [锁定] 的门");
            Door { _marker: std::marker::PhantomData }
        }

        // 重要：这里的参数是 self（获取所有权），而不是 &self
        // 这模拟了 C++ 中的 std::move(*this)，但在 Rust 中它是语言强制的
        fn unlock(self) -> Door<Unlocked> {
            println!("    门已 [解锁]");
            // 返回一个新状态的对象。旧的 Door<Locked> 在此函数结束时被销毁 (Dropped)
            Door { _marker: std::marker::PhantomData }
        }
    }

    // --- 状态 2: 只有在 Unlocked 状态下才能开门 ---
    impl Door<Unlocked> {
        // &self 表示只读借用，开门操作不会改变/消耗门的所有权
        fn open(&self) {
            println!("    成功 [打开] 了门！");
        }

        // 再次获取 self 所有权，将 Unlocked 门消耗掉，转回 Locked 状态
        fn lock(self) -> Door<Locked> {
            println!("    门已重新 [锁定]");
            Door { _marker: std::marker::PhantomData }
        }
    }

    // --- 状态机流转展示 ---
    let locked_door = Door::new();
    
    // ❌ 编译错误验证：
    // locked_door.open(); 
    // ^^^ 报错：no method named `open` found for struct `Door<Locked>`
    
    // ✅ 合法转换
    let unlocked_door = locked_door.unlock(); 
    
    // ❌ 编译错误验证（所有权已被转移）：
    // let x = locked_door; 
    // ^^^ 报错：use of moved value: `locked_door`
    // 这保证了你无法对同一个门进行“双重解锁”或在解锁后仍持有旧状态
    
    unlocked_door.open(); // 合法
    let _locked_again = unlocked_door.lock(); // 再次转移回 Locked
    
    println!("    状态机流转完成，且在编译期保证了绝对的安全。");
}

pub fn run_examples() {
    println!("=== 高级类型系统 ===");
    gat_examples();
    hrtb_examples();
    typestate_pattern();
    println!("===================\n");
}
