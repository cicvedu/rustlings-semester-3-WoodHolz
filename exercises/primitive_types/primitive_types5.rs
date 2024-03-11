// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// 元组 模式匹配 解构
fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    print!("{} is {} years old.", name, age);
}
