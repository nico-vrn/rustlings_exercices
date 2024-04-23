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
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    // Exemple d'utilisation de la macro sans paramètre.
    my_macro!();
    // Exemple d'utilisation de la macro avec un paramètre numérique.
    my_macro!(7777);
}
