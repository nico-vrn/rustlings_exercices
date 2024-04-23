// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) -> () {
    let tx_clone = Arc::clone(&tx);
    let first_half_thread = thread::spawn(move || {
        for val in q.first_half {
            println!("sending {:?}", val);
            let tx = tx_clone.lock().unwrap();
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let second_half_thread = thread::spawn(move || {
        for val in q.second_half {
            println!("sending {:?}", val);
            let tx = tx.lock().unwrap();
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    first_half_thread.join().unwrap();
    second_half_thread.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, Arc::clone(&tx));

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
