// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let expected = 42; // Définissez la valeur attendue ici
        let actual = 42; // Définissez la valeur actuelle ici
        assert_eq!(expected, actual);
    }
}
