// Assignment 4
use std::time::Duration;
use rand::Rng;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;
    
fn main() {

// Number of items to produce
const ITEM_COUNT: usize = 20;
const PRODUCER_COUNT: usize = 2;
const CONSUMER_COUNT: usize = 3;

// TODO: Create a channel for sending numbers
let (tx, rx) = mpsc::channel();
let rx = Arc::new(Mutex::new(rx));
let mut handles = vec![];

// TODO: Create 2 producer threads
for id in 0..PRODUCER_COUNT {
    let tx_clone = tx.clone();
    let items_per_producer = ITEM_COUNT / PRODUCER_COUNT;

    let handle = thread::spawn(move || {
        producer(id, tx_clone, items_per_producer);

    });
    handles.push(handle);
}

// TODO: Create 3 consumer threads

for id in 0..CONSUMER_COUNT {
    let rx_clone = Arc::clone(&rx);

    let handle = thread::spawn(move || {
        consumer(id, rx_clone);
    });
    handles.push(handle);
}

 // Termination signal for each consumer
 for _ in 0..CONSUMER_COUNT {
    let tx_clone = tx.clone();
    tx_clone.send(TERMINATION_SIGNAL).unwrap();
 }
// TODO: Wait for all threads to finish
for handle in handles {
    handle.join().unwrap();
}

println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
// TODO: Generate random numbers and send them to the channel
// When finished, producer should NOT send termination signal
let mut rng = rand::thread_rng();
for i in 0..item_count {
    let value = rng.gen_range(1..=100);
    println!("Producer {} sending value: {}", id, value);
    tx.send(value).unwrap();
    thread::sleep(Duration::from_millis(100));
}
    println!("Producer {} finished producing {} items.", id, item_count);

}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
// TODO: Receive numbers from the channel and process them
// Break the loop when receiving the termination signal
    loop {
        let value = {
            let lock = rx.lock().unwrap();
            lock.recv().unwrap()
        };
        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal. Exiting.", id);
            break;
        }
        println!("Consumer {} processing value: {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }

}