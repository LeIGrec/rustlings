// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

Ce programme Rust définit une structure générique Wrapper<T> qui encapsule une valeur de type T. La structure implémente une méthode new() qui crée une nouvelle instance de Wrapper en prenant la valeur en argument.

Les tests unitaires vérifient le comportement de la structure Wrapper, store_u32_in_wrapper() vérifie qu'un entier u32 peut être stocké dans un Wrapper et que sa valeur peut être récupérée correctement.

*/

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
