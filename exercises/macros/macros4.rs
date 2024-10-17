// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


mod macros {
    #[macro_export]  // 导出宏到全局命名空间
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();  // 现在可以在全局作用域中使用这个宏
}

