// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

/// La fonction `main` est le point d'entrée du programme. Dans ce programme, elle appelle la fonction `square`
/// avec un argument de 3, puis imprime le résultat dans la console.
/// 
/// La fonction `square` prend un nombre entier de type `i32` en entrée et retourne le carré de ce nombre.
/// 
/// # Arguments
/// 
/// * `num` - Un entier de type `i32` pour lequel nous voulons calculer le carré.
/// 
/// # Retour
/// 
/// La fonction retourne le carré de l'argument `num`.
/// 
/// # Exemple
/// 
/// ```rust
/// fn main() {
///     let answer = square(3);
///     println!("Le carré de 3 est {}", answer);
/// }
/// 
/// fn square(num: i32) -> i32 {
///     num * num 
/// }
/// ```
/// 
/// Ce programme imprime "Le carré de 3 est 9" dans la console lorsqu'il est exécuté.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

// Définition de la fonction square avec un type de retour i32
fn square(num: i32) -> i32 {
    num * num 
}

