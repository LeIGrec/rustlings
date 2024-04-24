// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

En Rust lorsqu'on déclare une constante avec le mot clé const, 
on doit spécifier le type de la constante. Le compilateur Rust doit connaitre le type de chaque variable au moment de la compilation.

*/

const  NUMBER: i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
