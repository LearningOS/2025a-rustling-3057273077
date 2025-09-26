// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


// 宏名称添加 `!` 后缀（关键修复）
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 调用宏时必须加上 `!`（关键修复）
    my_macro!();
}
