// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

Ce programme Rust définit une structure Point et utilise une variable y de type Option<Point> dans la fonction main(). 

Cependant, la variable y n'est pas utilisée après l'expression match.

Pour corriger ce problème, il suffit d'ajouter un point-virgule ";" à la fin de la ligne y pour indiquer que la variable est utilisée en tant qu'expression, même si sa valeur n'est pas utilisée. Cela éliminera l'avertissement du compileur concernant la variable inutilisée.

*/

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
