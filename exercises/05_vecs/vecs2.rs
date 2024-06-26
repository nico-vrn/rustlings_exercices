// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

/// Cette fonction `vec_loop` prend un vecteur mutable d'entiers 32 bits et multiplie chaque élément par 2.
/// 
/// # Arguments
/// 
/// * `v` - Un vecteur mutable d'entiers 32 bits.
/// 
/// # Retour
/// 
/// Un vecteur d'entiers 32 bits où chaque élément est multiplié par 2.
/// 

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> { // Prend un vecteur mutable d'entiers 32 bits
    for num in v.iter_mut() {
        *num *= 2; // Multiplie chaque élément par 2
    }
    v
}

/// Cette fonction `vec_map` prend une référence à un vecteur d'entiers 32 bits et retourne un nouveau vecteur où chaque élément est multiplié par 2.
/// 
/// # Arguments
/// 
/// * `v` - Une référence à un vecteur d'entiers 32 bits.
/// 
/// # Retour
/// 
/// Un vecteur d'entiers 32 bits où chaque élément est multiplié par 2.

fn vec_map(v: &Vec<i32>) -> Vec<i32> { // Prend une référence à un vecteur d'entiers 32 bits
    let mut new_vec = Vec::new();
    for num in v.iter() {
        new_vec.push(num * 2); // Ajoute chaque élément multiplié par 2 au nouveau Vec
    }
    new_vec
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() { // Teste la fonction vec_loop
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() { // Teste la fonction vec_map
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
