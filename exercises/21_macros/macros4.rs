// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

/// Une macro personnalisée pour afficher des messages dans la console.
/// 
/// Cette macro démontre l'utilisation de la surcharge pour permettre différentes formes d'appels.
/// Elle supporte deux formes d'utilisation:
/// 
/// - `my_macro!()` affiche un message standard.
/// - `my_macro!(expression)` affiche un message formaté avec la valeur de l'expression.

#[rustfmt::skip]
macro_rules! my_macro { // Définit la macro my_macro
    () => { // Si la macro est appelée sans paramètre
        println!("Check out my macro!"); // Affiche un message standard
    };
    ($val:expr) => { // Si la macro est appelée avec un paramètre
        println!("Look at this other macro: {}", $val); // Affiche un message formaté avec la valeur de l'expression
    };
}

fn main() {
    // Exemple d'utilisation de la macro sans paramètre.
    my_macro!();
    // Exemple d'utilisation de la macro avec un paramètre numérique.
    my_macro!(7777);
}
