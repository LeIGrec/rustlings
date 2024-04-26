// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

La macro à était mal définie, il manquait un ; ligne 18

La macro a deux variantes : une sans argument qui affiche un message, et une avec un argument $val qui est affiché dans le message.

*/

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
