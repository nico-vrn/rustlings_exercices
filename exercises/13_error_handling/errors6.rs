// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.

//! Ce programme définit une fonction `parse_pos_nonzero` qui prend une chaîne de caractères en entrée et tente de la convertir en un entier non nul positif. Si la conversion échoue ou si l'entier est négatif ou nul, des erreurs appropriées sont renvoyées.

use std::num::ParseIntError;

/// Erreur personnalisée pour la conversion de chaînes en entiers non nuls positifs.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    /// Crée une instance de l'erreur à partir d'une erreur de création.
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }

    /// Crée une instance de l'erreur à partir d'une erreur de conversion en entier.
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

/// Fonction pour convertir une chaîne en un entier non nul positif.
fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

/// Structure représentant un entier non nul positif.
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

/// Erreurs possibles lors de la création d'un entier non nul positif.
#[derive(PartialEq, Debug)]
enum CreationError { 
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    /// Crée une nouvelle instance d'entier non nul positif.
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative), // Si l'entier est négatif
            x if x == 0 => Err(CreationError::Zero), // Si l'entier est nul
            x => Ok(PositiveNonzeroInteger(x as u64)), // Sinon, crée un nouvel entier non nul positif
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
