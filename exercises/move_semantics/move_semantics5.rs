// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


// 可变引用在任何时候都是唯一的
fn main() {
    let mut x = 100;
    // y 可变引用
    let y = &mut x;
    *y += 100;
    // z 可变引用 从 y 转到 z 
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
