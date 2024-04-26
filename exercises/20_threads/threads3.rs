// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

/*
WRITE UP:

Ce programme implémente un système de file d'attente concurrente en utilisant des threads et des canaux de communication (mpsc::channel). 

La structure Queue contient une longueur et deux vecteurs représentant les deux moitiés de la file d'attente.

La fonction send_tx() crée deux threads qui envoient les éléments de chaque moitié de la file d'attente via le canal d'envoi (tx).

Les références au canal sont clonées en utilisant Arc pour permettre un accès concurrentiel sécurisé.

Dans la fonction main(), un canal est créé et la file d'attente est envoyée aux threads via send_tx().

Le récepteur (rx) récupère les éléments envoyés et vérifie que le nombre total reçu correspond à la longueur de la file d'attente.

*/
use std::sync::mpsc;
use std::sync::Arc;
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

fn send_tx(q: Queue, tx: Arc<mpsc::Sender<u32>>) -> () {
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);
    
    thread::spawn(move || {
        for val in q.first_half {
            println!("sending {:?}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in q.second_half {
            println!("sending {:?}", val);
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let tx = Arc::new(tx);

    send_tx(queue, Arc::clone(&tx));

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}

