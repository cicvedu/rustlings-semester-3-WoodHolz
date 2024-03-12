// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    /* 1 */
    // let mut shopping_list: Vec<&str> = Vec::new();
    /* 2 */
    let mut shopping_list = Vec::new();
    shopping_list.push("milk");
    // shopping_list.push("1");
    // println!("{:?}", shopping_list.pop());
}
