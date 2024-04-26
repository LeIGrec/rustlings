// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

Ce programme Rust définit une structure Book avec deux champs author et title de type String.

Pour résoudre le problème de durée de vie, la solution choisie est de changer le type des champs en String car les String sont allouées sur le tas et n'ont donc pas de problème de durée de vie.

Dans la fonction main(), des instances de String sont créées pour name et title, puis utilisées pour initialiser les champs correspondants de book. Enfin, les valeurs de title et author sont affichées.

*/

struct Book {
    author: String,
    title: String,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: name, title: title };

    println!("{} by {}", book.title, book.author);
}
