// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

/// Représentation des commandes possibles à appliquer sur les chaînes de caractères.
///
/// Les commandes incluent:
/// - `Uppercase`: Convertit une chaîne en majuscules.
/// - `Trim`: Supprime les espaces en début et en fin de chaîne.
/// - `Append`: Ajoute le mot "bar" à la chaîne un nombre spécifié de fois.
/// 
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    /// Transforme un vecteur de tuples contenant des chaînes et des commandes en un vecteur de chaînes transformées.
    ///
    /// Cette fonction prend chaque tuple du vecteur d'entrée, applique la commande spécifiée à la chaîne,
    /// et ajoute le résultat au vecteur de sortie.
    ///
    /// # Arguments
    /// * `input` - Vecteur de tuples `(String, Command)`, où chaque `String` est une chaîne à transformer
    /// et chaque `Command` est l'opération à appliquer sur cette chaîne.
    ///
    /// # Returns
    /// Retourne un vecteur de `String` contenant les résultats des transformations.
    ///
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(times) => {
                    let appended_string = format!("{}{}", string, "bar".repeat(*times));
                    output.push(appended_string);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Tests pour vérifier que la fonction `transformer` fonctionne correctement.
    #[test]
    fn it_works() {
        let output = my_module::transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}

