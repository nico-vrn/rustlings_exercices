// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

//! Ce programme définit une structure `Book` qui représente un livre avec un auteur et un titre.
//! Il crée une instance de `Book` en utilisant des chaînes de caractères pour l'auteur et le titre.
//! il affiche le titre et l'auteur du livre créé.

/// Représente un livre avec un auteur et un titre.
struct Book {
    author: String, 
    title: String,  
}

fn main() {
    // Création de chaînes de caractères pour l'auteur et le titre du livre.
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // Création d'un livre avec les données d'auteur et de titre.
    let book = Book { author: name, title: title };

    // Affichage du titre et de l'auteur du livre.
    println!("{} by {}", book.title, book.author);
}

