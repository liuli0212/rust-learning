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
