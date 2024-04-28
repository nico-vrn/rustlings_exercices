    // quiz3.rs
    //
    // This quiz tests:
    // - Generics
    // - Traits
    //
    // An imaginary magical school has a new report card generation system written
    // in Rust! Currently the system only supports creating report cards where the
    // student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
    // school also issues alphabetical grades (A+ -> F-) and needs to be able to
    // print both types of report card!
    //
    // Make the necessary code changes in the struct ReportCard and the impl block
    // to support alphabetical report cards. Change the Grade in the second test to
    // "A+" to show that your changes allow alphabetical grades.
    //
    // Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


/// Trait définissant une interface commune pour différents formats de notes.
///
/// Ce trait garantit que tout format de note, qu'il soit numérique ou alphabétique,
/// peut être imprimé sous forme de chaîne de caractères. Les implémentations de ce trait
/// doivent fournir une méthode `print_grade` qui convertit la note en un format de chaîne
/// affichable.
/// 
    pub trait Grade {
        fn print_grade(&self) -> String;
    }
/// Implémentation de `Grade` pour les nombres à virgule flottante.
    impl Grade for f32 {
        fn print_grade(&self) -> String {
            self.to_string()
        }
    }
/// Implémentation de `Grade` pour les tranches de chaînes statiques, permettant des notes alphabétiques.
    impl Grade for &'static str {
        fn print_grade(&self) -> String {
            self.to_string()
        }
    }

    pub struct ReportCard<T: Grade> { //Structure de bulletin de notes
        pub grade: T,
        pub student_name: String,
        pub student_age: u8,
    }

/// Un bulletin de notes contenant une note, le nom de l'étudiant et son âge.
///
/// La structure `ReportCard` est générique sur le type de note `T`, qui doit implémenter
/// le trait `Grade`. Cette conception permet au bulletin de prendre en charge tout type de note
/// pouvant être représenté sous forme de chaîne à des fins de rapport.
///
    impl<T: Grade> ReportCard<T> { //Implémentation de ReportCard
        pub fn print(&self) -> String { //Fonction pour imprimer le bulletin de notes
            format!(
                "{} ({}) - achieved a grade of {}",
                &self.student_name,
                &self.student_age,
                &self.grade.print_grade()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn generate_numeric_report_card() {
            let report_card = ReportCard {
                grade: 2.1,
                student_name: "Tom Wriggle".to_string(),
                student_age: 12,
            };
            assert_eq!(
                report_card.print(),
                "Tom Wriggle (12) - achieved a grade of 2.1"
            );
        }

        #[test]
        fn generate_alphabetic_report_card() {
            let report_card = ReportCard {
                grade: "A+",
                student_name: "Gary Plotter".to_string(),
                student_age: 11,
            };
            assert_eq!(
                report_card.print(),
                "Gary Plotter (11) - achieved a grade of A+"
            );
        }
    }
