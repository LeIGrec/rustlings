// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

⚠️  Compiling of exercises/12_options/options3.rs failed! Please try again. Here's the output:
error[E0382]: use of partially moved value: `y`
  --> exercises/12_options/options3.rs:23:5
   |
20 |         Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
   |              - value partially moved here
...
23 |     y; // Fix without deleting this line.
   |     ^ value used here after partial move
   |
   = note: partial move occurs because value has type `Point`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
   |
20 |         Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
   |              +++

error: aborting due to 1 previous error

J'ai juste suivis les instructions de l'erreur et j'ai ajouté le mot clé ref dans le match pour éviter de déplacer la valeur de y.

ref permet de faire une référence à la valeur sans la déplacer.
ça ressemble à un pointeur mais ce n'est pas un pointeur.
Point n'implémente pas le trait Copy donc on peut pas le copier comme ça avec clone().
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
