// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

/*

WRITE UP:

Il faut crée un tuple numbers contenant 1, 2 et 3, puis l'indexation par point pour accéder au deuxième élément (index 1) et le stocker dans second.

Enfin, nous vérifions que second est égal à 2 à l'aide de assert_eq!

Ce test montre l'utilisation des tuples et de l'indexation en Rust

*/

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
