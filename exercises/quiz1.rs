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

// Put your function here!

/// Calcule le prix total pour une commande de pommes en fonction de la quantité achetée.
///
/// Le prix des pommes est déterminé par les règles suivantes :
/// - Chaque pomme coûte 2 rustbucks.
/// - Si la quantité achetée dépasse 40 pommes, le prix par pomme tombe à 1 rustbuck.
/// # Arguments
/// * `quantity` - Le nombre de pommes achetées, un entier non signé (`u32`).
///
/// # Returns
/// Retourne le coût total de l'achat en rustbucks (`u32`).
///
fn calculate_price_of_apples(quantity: u32) -> u32 {
    if quantity > 40 {
        quantity
    } else {
        quantity * 2
    }
}

// Don't modify this function!
// Cette fonction de test vérifie la logique de `calculate_price_of_apples` pour
// s'assurer qu'elle respecte les conditions de prix données.
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
