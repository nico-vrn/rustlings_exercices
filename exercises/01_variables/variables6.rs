// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

/// La constante `NUMBER` est définie avec une valeur de 3 de type `i32`. Les constantes sont des variables 
/// immuables dont la valeur ne peut pas être modifiée tout au long de l'exécution du programme.
/// 
/// Ce programme imprime simplement la valeur de la constante `NUMBER` dans la console lorsqu'il est exécuté.

const NUMBER: i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}

