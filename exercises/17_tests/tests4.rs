// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

//! Ce module définit une structure `Rectangle` représentant un rectangle avec une largeur et une hauteur.
//! Les fonctions de test vérifient le comportement de la méthode `new` dans différentes situations.

/// Représente un rectangle avec une largeur et une hauteur.
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    /// Crée un nouveau rectangle avec la largeur et la hauteur spécifiées.
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // Vérifie si le rectangle a la largeur et la hauteur spécifiées dans le constructeur.
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // Vérifie si le programme panique lors de la tentative de création d'un rectangle avec une largeur négative.
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // Vérifie si le programme panique lors de la tentative de création d'un rectangle avec une hauteur négative.
        let _rect = Rectangle::new(10, -10);
    }
}
