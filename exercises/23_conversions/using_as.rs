// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

Ce programme définit une fonction average() qui calcule la moyenne d'une tranche de f64.

Un problème survient lors de la division par values.len() car celui-ci retourne un usize incompatible avec f64. 

La seule solution est de convertir explicitement values.len() en f64 avec as f64 pour permettre la division.

La fonction main() crée un tableau de f64 et appelle average() pour afficher le résultat. Un test unitaire vérifie le bon fonctionnement de average().

*/

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();

    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
