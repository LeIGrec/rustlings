// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

Dans ce programme Rust, nous déclarons une constante nommée NUMBER de type i32 (entier signé 32 bits) et lui attribuons la valeur 3.

La déclaration de constante utilise le mot-clé const suivi du nom de la constante, des deux-points et du type de données. Ensuite, le signe égal (=) est utilisé pour assigner une valeur à la constante.

Dans la fonction main(), nous utilisons la macro println! pour afficher la valeur de la constante NUMBER en utilisant le marqueur de format {}.

*/

const  NUMBER: i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
