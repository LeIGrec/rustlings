// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)

/*
WRITE UP:

Ce code définit une fonction calculate_price_of_apples qui prend en entrée une quantité d'apples (quantity) de type i32 et renvoie le prix total de ces pommes. La logique est la suivante

Si la quantité est supérieure à 40, la fonction renvoie simplement la quantité

Sinon, la fonction multiplie la quantité par 2

Le code contient également un test unitaire verify_test qui vérifie le bon fonctionnement de la fonction calculate_price_of_apples pour différentes valeurs de quantity.

*/


// Put your function here!
fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity > 40 {
        quantity
    } else {
        quantity * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
