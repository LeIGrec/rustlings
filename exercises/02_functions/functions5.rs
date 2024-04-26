// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

/*

WRITE UP:

Ce programme définit deux fonctions : main() et square().

La fonction main() appelle square(3) qui calcule le carré de 3 et stocke le résultat dans la variable answer.

main() affiche "The square of 3 is 9" en utilisant la valeur de answer.

La fonction square() prend un argument num de type i32, le multiplie par lui-même et renvoie le résultat sans utiliser le mot-clé return.

*/

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
