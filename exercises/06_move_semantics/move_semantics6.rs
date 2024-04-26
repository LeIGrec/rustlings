// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

/*
WRITE UP:
    
Ce programme montre comment les références et la propriété fonctionnent en Rust.

La fonction get_char() emprunte une référence à &data, tandis que string_uppercase() prend la propriété de data.

Cela permet de s'assurer que les données sont utilisées de manière sûre.

*/

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
