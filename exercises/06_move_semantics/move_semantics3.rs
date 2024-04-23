// move_semantics3.rs
//
// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let mut vec0 = vec![22, 44, 66];

    fill_vec(&mut vec0);

    assert_eq!(vec0, vec![22, 44, 66, 88]); 
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

