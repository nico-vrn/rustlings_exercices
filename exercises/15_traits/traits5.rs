// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

//! Ce programme définit deux traits, `SomeTrait` et `OtherTrait`, avec une méthode par défaut qui renvoie toujours `true`.
//! Il définit également deux structures, `SomeStruct` et `OtherStruct`, et implémente les deux traits pour chacune d'entre elles.
//! La fonction `some_func` prend un argument de type générique `T` qui doit implémenter à la fois `SomeTrait` et `OtherTrait`.
//! La fonction vérifie si les méthodes `some_function` et `other_function` renvoient toutes deux `true` pour l'élément donné.

/// Un trait avec une méthode par défaut qui renvoie toujours `true`.
pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

/// Un autre trait avec une méthode par défaut qui renvoie toujours `true`.
pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

/// Une structure vide.
struct SomeStruct {}

/// Une autre structure vide.
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

/// Une fonction qui vérifie si les méthodes `some_function` et `other_function` renvoient toutes deux `true`.
fn some_func<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait,
{
    item.some_function() && item.other_function()
}

fn main() {
    // Appel de `some_func` avec une instance de `SomeStruct`.
    some_func(SomeStruct {});

    // Appel de `some_func` avec une instance de `OtherStruct`.
    some_func(OtherStruct {});
}
