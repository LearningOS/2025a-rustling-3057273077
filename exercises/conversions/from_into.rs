// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)] // 仅一次派生 Debug
struct Person {
    name: String,
    age: usize,
}

// 仅一次实现 Default  trait（原代码已存在，无需重复）
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 只补充这部分 From<&str> 的实现，不要重复定义结构体或 Default！
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 1. 空字符串直接返回默认值
        if s.is_empty() {
            return Person::default();
        }

        // 2. 按逗号分割字符串，收集为切片
        let parts: Vec<&str> = s.split(',').collect();

        // 3. 分割后必须恰好有 2 部分（姓名+年龄），否则返回默认
        if parts.len() != 2 {
            return Person::default();
        }

        // 4. 提取姓名，去除前后空格后检查是否为空
        let name = parts[0].trim();
        if name.is_empty() {
            return Person::default();
        }

        // 5. 提取年龄字符串，去除空格后尝试解析为 usize
        let age_str = parts[1].trim();
        let age = match age_str.parse::<usize>() {
            Ok(num) => num, // 解析成功则获取年龄
            Err(_) => return Person::default(), // 解析失败返回默认
        };

        // 所有检查通过，返回自定义 Person 实例
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let p1 = Person::from("Mark,20");
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}