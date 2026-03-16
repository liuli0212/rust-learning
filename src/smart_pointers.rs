//! 专用智能指针与零拷贝 (Smart Pointers & Zero-Copy)
//!
//! 展示写时复制 (Cow)、内存钉扎 (Pin) 以及高性能的 Zero-copy 强转。

use std::borrow::Cow;
use std::pin::Pin;
use std::marker::PhantomPinned;
use zerocopy::{FromBytes, IntoBytes, KnownLayout, Immutable};

/// 1. Cow (Copy-on-Write)
/// 适用于“可能修改，也可能不修改”的场景，避免不必要的内存分配。
pub fn cow_examples() {
    println!("  === Cow (写时复制) ===");

    fn process_string(input: &str) -> Cow<str> {
        if input.contains("error") {
            // 需要修改时，转换为 owned (String)，产生堆分配
            let replaced = input.replace("error", "warning");
            Cow::Owned(replaced)
        } else {
            // 不需要修改时，直接返回 borrowed (&str)，零成本
            Cow::Borrowed(input)
        }
    }

    let s1 = process_string("Everything is fine");
    println!("    无修改场景: {}", s1);
    
    // 我们还可以检查它到底发生了复制没有
    match s1 {
        Cow::Borrowed(_) => println!("      (它是借用的)"),
        Cow::Owned(_) => println!("      (它是独占/新分配的)"),
    }

    let s2 = process_string("This is an error message");
    println!("    有修改场景: {}", s2);
    match s2 {
        Cow::Borrowed(_) => println!("      (它是借用的)"),
        Cow::Owned(_) => println!("      (它是独占/新分配的)"),
    }
}

/// 2. Pin (内存钉扎)
/// 用于保证对象在内存中的地址永远不会改变。
/// 这在实现自引用结构（Self-referential structs）和异步 Future 中至关重要。
pub fn pin_examples() {
    println!("\n  === Pin (内存钉扎) ===");

    // 一个带有 PhantomPinned 的结构体，意味着它 !Unpin (即它不能安全地在内存中被移动)
    struct SelfReferential {
        data: String,
        pointer_to_data: *const String,
        _pin: PhantomPinned, // 标记这个类型不应该被 move
    }

    impl SelfReferential {
        fn new(txt: &str) -> Pin<Box<Self>> {
            let res = SelfReferential {
                data: String::from(txt),
                pointer_to_data: std::ptr::null(),
                _pin: PhantomPinned,
            };
            
            // 将其放入堆上并 Pin 住
            let mut boxed = Box::pin(res);
            
            // 现在它的地址固定了，可以安全地创建指向自己的指针
            let self_ptr: *const String = &boxed.data;
            unsafe {
                // 我们必须绕过 Pin 才能修改它内部的值
                let mut_ref: Pin<&mut SelfReferential> = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).pointer_to_data = self_ptr;
            }
            
            boxed
        }
        
        fn print_data(&self) {
            unsafe {
                println!("    数据: {}, 通过自引用指针读取: {}", self.data, &*self.pointer_to_data);
            }
        }
    }

    let pinned_struct = SelfReferential::new("I am pinned!");
    pinned_struct.print_data();
    println!("    如果你尝试 std::mem::swap 两个 pinned_struct，编译器会报错拦截，从而防止悬垂指针。");
}

/// 3. Zero-Copy (零拷贝解析)
/// 像 C 语言中那样直接把字节流强制转换为结构体，但完全安全。
pub fn zerocopy_examples() {
    println!("\n  === Zero-Copy 解析 ===");

    // 必须有 repr(C) 和相关的 derive，以确保内存布局可控且所有位模式都是合法的
    #[repr(C)]
    #[derive(Debug, FromBytes, IntoBytes, KnownLayout, Immutable, PartialEq)]
    struct NetworkPacket {
        packet_id: u32,
        payload_size: u16,
        flags: u16,
        data: [u8; 8], // 严格占用 16 字节
    }

    // 假设这是我们从网络或磁盘读取的一大段字节流
    let raw_bytes: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, // id: 1 (小端序)
        0x08, 0x00,             // size: 8
        0xff, 0x00,             // flags: 255
        0x41, 0x42, 0x43, 0x44, // "ABCD"
        0x45, 0x46, 0x47, 0x48  // "EFGH"
    ];

    // 将字节数组直接作为结构体引用读取（零分配，零拷贝）
    // 注意：这里需要考虑端序问题，实际生产中常用 u32::from_le 等包装类型
    let packet_ref = NetworkPacket::ref_from_bytes(&raw_bytes).expect("字节流大小或对齐不匹配");
    
    println!("    从字节流零拷贝解析出的结构体:");
    println!("    ID: {}", packet_ref.packet_id);
    println!("    Size: {}", packet_ref.payload_size);
    println!("    Data: {:?}", std::str::from_utf8(&packet_ref.data).unwrap_or("error"));

    // 反向：结构体直接转回字节流
    let out_bytes = packet_ref.as_bytes();
    println!("    结构体转回的字节流 (前4个字节): {:?}", &out_bytes[..4]);
}

pub fn run_examples() {
    println!("=== 专用智能指针与零拷贝 ===");
    cow_examples();
    pin_examples();
    zerocopy_examples();
    println!("===========================\n");
}
