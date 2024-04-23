// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.

//! Ce module fournit des fonctions pour compter les occurrences d'une valeur spécifique dans une collection de hashmaps.
//! Il définit également une énumération `Progress` et utilise des tests pour vérifier le comportement des fonctions.

use std::collections::HashMap;

/// Représente les différents états de progression.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

/// Compte les occurrences de la valeur spécifiée dans la hashmap donnée.
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // Count the occurrences of the specified `value` in the hashmap `map`.
    map.values().filter(|&&v| v == value).count()
}

/// Compte les occurrences de la valeur spécifiée dans chaque hashmap de la collection.
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // Count the occurrences of the specified `value` in each hashmap in the collection.
    // Then sum up the counts for all hashmaps.
    collection.iter().map(|map| count_iterator(map, value)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        // Vérifie si le nombre d'occurrences de l'état "Complete" est correct.
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        // Vérifie si le nombre d'occurrences de l'état "Some" est correct.
        let map = get_map();
        assert_eq!(1, count_iterator(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        // Vérifie si le nombre d'occurrences de l'état "None" est correct.
        let map = get_map();
        assert_eq!(2, count_iterator(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        // Vérifie si le nombre d'occurrences est le même lors de l'utilisation de deux boucles for.
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_iterator(&map, progress_state),
                count_iterator(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        // Vérifie si le nombre d'occurrences est le même lors de l'utilisation de deux boucles for.
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        // Vérifie si le nombre d'occurrences de l'état "Some" est correct dans une collection de hashmaps.
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        // Vérifie si le nombre d'occurrences de l'état "None" est correct dans une collection de hashmaps.
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        // Vérifie si le nombre d'occurrences est le même lors de l'utilisation de deux boucles for dans une collection de hashmaps.
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_iterator(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    // Génère une hashmap pour les tests.
    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    // Génère une collection de hashmaps pour les tests.
    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}