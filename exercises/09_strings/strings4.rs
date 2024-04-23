// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

//! Ce programme illustre plusieurs façons de manipuler et d'afficher des chaînes de caractères en Rust.

/// Affiche une référence à une chaîne de caractères.
fn string_slice(arg: &str) {
    println!("{}", arg);
}

/// Affiche une chaîne de caractères propre.
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    /// Affiche une référence à une chaîne de caractères littérale.
    string_slice("blue");
    /// Affiche une chaîne de caractères créée à partir d'une chaîne littérale.
    string("red".to_string());
    /// Affiche une chaîne de caractères créée à partir d'une chaîne existante.
    string(String::from("hi"));
    /// Affiche une chaîne de caractères créée à partir d'une chaîne littérale avec la méthode `into()`.
    string("rust is fun!".into());
    /// Affiche une chaîne de caractères formatée.
    string(format!("Interpolation {}", "Station"));
    /// Affiche le premier caractère d'une chaîne de caractères référencée.
    string_slice(&String::from("abc")[0..1]);
    /// Affiche une chaîne de caractères sans les espaces au début et à la fin.
    string("  hello there ".trim().to_string());
    /// Affiche une chaîne de caractères avec un remplacement.
    string("Happy Monday!".replace("Mon", "Tues"));
    /// Affiche une chaîne de caractères convertie en minuscules.
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
