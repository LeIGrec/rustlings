// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

Pour corrige le problème de life time il y a 2 solutions.

La première rajouter le life time à la structure book 
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

La deuxième solution est de changer le type de author et title en String
Vu que les String sont des types qui sont alloués sur le tas, il n'y a pas de problème de life time
Cette solution est plus simple donc je vais la choisir

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
