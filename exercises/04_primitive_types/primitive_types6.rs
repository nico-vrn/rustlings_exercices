// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

/// Cette fonction de test `indexing_tuple` vérifie l'indexation d'un tuple pour obtenir son deuxième élément.
/// 
/// Elle crée un tuple de nombres (1, 2, 3) et utilise l'indexation du tuple pour obtenir le deuxième élément, qui devrait être égal à 2.
/// 
/// # Remarque
/// 
/// Si l'indexation échoue et que le deuxième élément du tuple n'est pas égal à 2, un message d'erreur est affiché.


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
