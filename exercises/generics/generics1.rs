// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    // 将 <?> 替换为 &str，指定向量存储字符串切片
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk"); // 现在类型匹配，可正常编译
}
