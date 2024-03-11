// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.


/**
 * 这是错误的
 * const NUMBER = 3u32;
 */

// const 常量必须显式表明类型
const NUMBER: u32 = 3;
fn main() {
    print!("Number {}", NUMBER);
}
