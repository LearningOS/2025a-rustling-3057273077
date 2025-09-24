// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.



// 为结构体添加生命周期参数 'a，并将其应用到两个 &str 引用
struct Book<'a> {
    author: &'a str,  // 标注 author 的生命周期与结构体一致
    title: &'a str,   // 标注 title 的生命周期与结构体一致
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // Book 实例的生命周期由 name 和 title 决定（取两者中较短的存活时间）
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
