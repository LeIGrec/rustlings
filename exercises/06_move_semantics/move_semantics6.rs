// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

/*
WRITE UP:
    
La fonction get_char prend ownership de la String data, donc on ne peut plus l'utiliser après. 
Donc premier fix : get_char(&data); et get_char(data: &String); J'ai réjouter les & pour passer une référence à la fonction get_char

Puis la fonction string_uppercase on doit remove les & pour prendre ownership de la String data.
(Enfin c'est ce que je pense, je suis pas sûr de moi)

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
