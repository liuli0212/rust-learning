//! 深入内存布局与对齐 (Memory Layout & Alignment)
//!
//! 这个模块针对 C++ 开发者展示 Rust 在底层内存布局、对齐规则、
//! 胖指针 (Fat Pointers) 以及动态大小类型 (DST) 上的机制。

use std::mem::{align_of, size_of};

/// 1. 结构体内存对齐与 Padding (对比 C++)
pub fn layout_and_padding() {
    println!("  === 结构体内存对齐与 Padding ===");

    // Rust 默认布局 (repr(Rust))：编译器可以自由重排字段以优化内存
    struct DefaultStruct {
        a: u8,
        b: u32,
        c: u16,
    }

    // C 兼容布局 (repr(C))：严格按照声明顺序，和 C/C++ 的 struct 一样
    #[repr(C)]
    struct CStruct {
        a: u8,
        // padding: 3 bytes
        b: u32,
        c: u16,
        // padding: 2 bytes (为了整体对齐到 4 bytes)
    }

    // 紧凑布局：禁用所有 padding，可能会导致未对齐访问惩罚
    #[repr(packed)]
    struct PackedStruct {
        a: u8,
        b: u32,
        c: u16,
    }

    println!("    DefaultStruct: size = {}, align = {}", size_of::<DefaultStruct>(), align_of::<DefaultStruct>());
    println!("    CStruct:       size = {}, align = {}", size_of::<CStruct>(), align_of::<CStruct>());
    println!("    PackedStruct:  size = {}, align = {}", size_of::<PackedStruct>(), align_of::<PackedStruct>());
}

/// 2. 动态大小类型 (DST) 与胖指针 (Fat Pointers)
pub fn dst_and_fat_pointers() {
    println!("\n  === 动态大小类型与胖指针 ===");

    let array: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &array; // slice 是一个胖指针，包含指针和长度

    // 瘦指针：只包含一个内存地址（例如指向明确大小数组的引用）
    println!("    瘦指针大小 ( &array ): {} bytes", size_of::<&[i32; 3]>());
    
    // 胖指针：包含地址 + 长度元数据
    println!("    胖指针大小 ( slice ): {} bytes", size_of::<&[i32]>());

    // 手动拆解胖指针（使用 unsafe 观察底层布局）
    unsafe {
        // [指针(8 bytes), 长度(8 bytes)]
        let raw_fat_ptr: [usize; 2] = std::mem::transmute(slice);
        println!("    解剖胖指针 - 内存地址: 0x{:x}, 长度: {}", raw_fat_ptr[0], raw_fat_ptr[1]);
    }

    // 另一种胖指针：Trait Object (指向数据的指针 + 指向 vtable 的指针)
    trait DoSomething { fn do_it(&self); }
    impl DoSomething for i32 { fn do_it(&self) {} }

    let value: i32 = 42;
    let trait_obj: &dyn DoSomething = &value;

    println!("    Trait Object 胖指针大小: {} bytes", size_of::<&dyn DoSomething>());
    unsafe {
        let raw_trait_obj: [usize; 2] = std::mem::transmute(trait_obj);
        println!("    解剖 Trait Object - 数据地址: 0x{:x}, vtable地址: 0x{:x}", raw_trait_obj[0], raw_trait_obj[1]);
    }
}

/// 3. 手动控制对齐 (SIMD/Cache Line)
pub fn manual_alignment() {
    println!("\n  === 手动控制对齐 ===");

    // 强制按缓存行 (通常是 64 字节) 对齐，常用于高并发避免伪共享 (False Sharing)
    #[repr(align(64))]
    struct CacheLineAligned {
        data: [i32; 4],
    }

    let aligned_data = CacheLineAligned { data: [1, 2, 3, 4] };
    let addr = &aligned_data as *const _ as usize;

    println!("    CacheLineAligned 结构体大小: {} bytes", size_of::<CacheLineAligned>());
    println!("    CacheLineAligned 结构体对齐: {} bytes", align_of::<CacheLineAligned>());
    println!("    内存地址: 0x{:x} (是否为64的倍数: {})", addr, addr % 64 == 0);
}

pub fn run_examples() {
    println!("=== 内存布局与底层机制 ===");
    layout_and_padding();
    dst_and_fat_pointers();
    manual_alignment();
    println!("=========================\n");
}
