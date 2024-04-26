// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

/*

Ce programme contenait plusieurs erreurs qui ont été corrigées

La ligne 23 qui faisait unwrap() sur my_option a été commentée car my_option étant None, il n'était pas possible de le déballer sans risquer une panique.

Une virgule manquait dans la déclaration de my_arr à la ligne 27, ce qui empêchait la compilation.

Pour échanger correctement les valeurs de value_a et value_b, la fonction std::mem::swap() a été utilisée

*/


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Remove unnecessary None and unwrap() call
    let my_empty_vec: Vec<i32> = Vec::new();
    vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see?");

    let mut value_a = 45;
    let mut value_b = 66;
    // Swap the values correctly
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}


