// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



// 无需手动导入 std::f32，Clippy 可能提示：未使用的导入（因为 f32::consts::PI 可直接访问）
// use std::f32;  // 注释或删除未使用的导入

// 正确导入 f32 模块
// 删除未使用的导入语句
// use std::f32;

fn main() {
    // 使用完整路径访问 PI 常量
    let pi = std::f32::consts::PI;
    let radius = 5.00f32;

    // 使用更简洁的方法调用语法
    let area = pi * radius.powi(2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}