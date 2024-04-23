// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// Définition de la structure `Book` avec des champs `author` et `title`.
struct Book {
    author: String, // L'auteur du livre.
    title: String,  // Le titre du livre.
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
