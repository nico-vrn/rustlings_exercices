// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(data.clone()); 

    println!("Last character: {}", last_char);

    let data = string_uppercase(&data); 
    println!("Uppercase string: {}", data);
}

// Devrait prendre la propriété
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Ne devrait pas prendre la propriété
fn string_uppercase(data: &String) -> String {
    data.to_uppercase()
}
