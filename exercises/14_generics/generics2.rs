// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

//! Ce programme définit une structure générique `Wrapper<T>` qui enveloppe une valeur de n'importe quel type `T`.
//! La méthode `new` permet de créer une nouvelle instance de `Wrapper`.

/// Une structure générique qui enveloppe une valeur de type `T`.
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    /// Crée une nouvelle instance de `Wrapper` avec la valeur spécifiée.
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        // Teste l'enveloppement d'un entier 32 bits non signé.
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // Teste l'enveloppement d'une chaîne de caractères.
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
