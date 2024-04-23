// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

//! Ce programme affiche le nombre de secondes écoulées depuis le 1er janvier 1970 00:00:00 UTC.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    /// Récupère le temps actuel sous forme de durée depuis l'époque UNIX.
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        /// Affiche le nombre de secondes écoulées depuis l'époque UNIX.
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
