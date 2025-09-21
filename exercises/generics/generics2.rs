// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



// 为 Wrapper 添加泛型参数 T，支持存储任意类型
struct Wrapper<T> {
    value: T, // value 字段类型改为泛型 T
}

impl<T> Wrapper<T> { // 为泛型结构体实现方法，需在 impl 后加 <T>
    // new 方法接收泛型类型 T 的参数，返回 Wrapper<T>
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        // 自动推导 Wrapper<u32>，value 类型为 u32
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // 自动推导 Wrapper<&str>，value 类型为 &str（字符串字面量类型）
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
