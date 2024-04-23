// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

//! Ce programme définit une structure Point représentant des coordonnées x et y, puis utilise une option pour stocker un point ou aucune valeur. Ensuite, il utilise un motif `if let` pour extraire les coordonnées du point s'il est présent.

/// Structure représentant des coordonnées x et y.
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    /// Option contenant un point
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    /// Utilisation d'un motif `if let` pour extraire les coordonnées du point
    if let Some(ref p) = y {
        println!("Co-ordinates are {},{} ", p.x, p.y);
    } else {
        panic!("no match!");
    }
    y; // Fix without deleting this line.
}

