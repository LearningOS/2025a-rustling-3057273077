//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // 这里的时间戳是 tests7 需要的 TEST_FOO 的值
    // 修改：用 rustc-env 指令设置 TEST_FOO=时间戳
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 修改：用 rustc-cfg 指令启用 pass 特征
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}