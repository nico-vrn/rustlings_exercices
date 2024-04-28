// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.

//! Ce programme définit une structure `Package` représentant un colis à expédier. Il fournit des méthodes pour créer un nouveau colis, vérifier s'il s'agit d'un envoi international, et calculer les frais de transport en fonction du poids du colis et du tarif par gramme.

#[derive(Debug)] 
struct Package { // Structure Package
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    /// Crée un nouveau colis avec les informations fournies.
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package { // Crée un nouveau colis avec les informations fournies
        if weight_in_grams < 10 {
            panic!("Can not ship a package with weight below 10 grams.") // On ne peut pas expédier un colis pesant moins de 10 grammes
        } else {
            Package { // Crée un nouveau colis
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }
    /// Vérifie si le colis est un envoi international.
    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }
    /// Calcule les frais de transport en fonction du tarif par gramme.
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        self.weight_in_grams * cents_per_gram
    }
}

#[cfg(test)] 
mod tests { // Tests
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country.clone(), recipient_country.clone(), 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country.clone(), recipient_country.clone(), 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country.clone(), recipient_country.clone(), 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
