// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

/// Cette fonction `animal_habitat` prend un nom d'animal en entrée et retourne l'habitat associé à cet animal.
/// 
/// # Arguments
/// 
/// * `animal` - Une référence à une chaîne de caractères représentant le nom de l'animal.
/// 
/// # Retour
/// 
/// La fonction retourne une référence statique (`&'static str`) représentant l'habitat de l'animal.


pub fn animal_habitat(animal: &str) -> &'static str { // Prend une référence à une chaîne de caractères
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" { // Si l'animal est un gopher
        2
    } else if animal == "snake" {   // Si l'animal est un serpent
        3
    } else {
        0
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}

