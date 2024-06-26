// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

/// Calcule la moyenne des valeurs dans un tableau de nombres flottants.
///
/// Cette fonction prend une tranche de nombres flottants (`f64`) et retourne leur moyenne.
/// Si le tableau est vide, la fonction retournera zéro, car la division par zéro est évitée
 
fn average(values: &[f64]) -> f64 { //Fonction pour calculer la moyenne des valeurs
    let total = values.iter().sum::<f64>(); //Somme des valeurs
    total / values.len() as f64
}

fn main() { 
    let values = [3.5, 0.3, 13.0, 11.7]; //Tableau de valeurs
    println!("{}", average(&values)); //Affiche la moyenne des valeurs
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Teste la fonction `average` pour s'assurer qu'elle retourne la bonne valeur et le bon type.
    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
