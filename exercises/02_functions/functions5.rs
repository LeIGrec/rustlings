// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:


En Rust tu as deux façons de retourner une valeur d'une fonction:
- en utilisant le mot clé return

fn square(num: i32) -> i32 {
    return num * num;
}
- en utilisant l'expression sans le mot clé return et prend pas de ; à la fin

fn square(num: i32) -> i32 {
    num * num
}


J'ai choisi la deuxième option car c'est plus rapide à écrire

*/

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
