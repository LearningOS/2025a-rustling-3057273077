// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 补全函数签名：输入是 Vec<(String, Command)>，输出是 Vec<String>
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // 补全输出类型：与返回值一致的 Vec<String>
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // 根据不同命令处理字符串，将结果存入 output
            match command {
                // 1. 处理 Uppercase：将字符串转为大写
                Command::Uppercase => output.push(string.to_uppercase()),
                // 2. 处理 Trim：去除字符串首尾空白（空格、制表符等）
                Command::Trim => output.push(string.trim().to_string()),
                // 3. 处理 Append(n)：在字符串后追加 n 次 "bar"
                Command::Append(n) => {
                    let appended = string.repeat(1) + &"bar".repeat(*n);
                    output.push(appended);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 导入 my_module 中的 transformer 函数（需用绝对/相对路径）
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),          // 输出 "HELLO"
            (" all roads lead to rome! ".into(), Command::Trim), // 输出 "all roads lead to rome!"
            ("foo".into(), Command::Append(1)),            // 输出 "foobar"（追加1次"bar"）
            ("bar".into(), Command::Append(5)),            // 输出 "barbarbarbarbarbar"（追加5次"bar"）
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}