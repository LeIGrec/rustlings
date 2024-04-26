// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

Ce programme définit deux traits SomeTrait et OtherTrait avec des méthodes par défaut.

Il définit également deux structures SomeStruct et OtherStruct qui implémentent les deux traits.

La signature de la fonction some_func doit être modifiée pour prendre un paramètre item qui implémente soit SomeTrait, soit OtherTrait, en utilisant la syntaxe impl SomeTrait + OtherTrait.

Ce programme illustre l'utilisation des traits en Rust pour définir des comportements communs pour différents types, ainsi que l'utilisation de la syntaxe impl Trait pour spécifier les contraintes sur les types de paramètres de fonction.

*/

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
