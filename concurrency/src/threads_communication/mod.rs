use std::sync::mpsc;
use std::thread;
use std::time::Duration; // multiple producer single consumer

pub fn main_threads_communication() {
    /* COMMUNICATION BETWEEN THREADS */

    /*********************************************************
     *
     *  CHANNELS (with Strings and single producer)
     *
     *********************************************************/

    let mut instrument = String::from("guitar");

    // Create a new channel
    // tx: variable used by the producer - rx: variable used by the consumer
    let (tx, rx) = mpsc::channel();

    // Create the producer thread join handler
    //   - Use 'move' in the closure to transfer the ownership of the 'instrumet' and 'tx' variables to the thread
    let luthier_thread = thread::spawn(move || {
        // Modify the String (this thread is the owner)
        instrument = String::from("Les Paul ") + &instrument;

        println!("I'm crafting a {}...", instrument);
        thread::sleep(Duration::from_millis(100));

        // Send the value to the channel (and transfer the ownership)
        tx.send(instrument).unwrap();
    });

    // Create the consumer thread join handler
    //   - Use 'move' in the closure to transfer the ownership of the 'rx' variable to the thread
    let musician_thread = thread::spawn(move || {
        let guitar = rx.recv().unwrap();
        println!("I have bought a {}!", guitar);
    });

    // Execute the threads
    luthier_thread.join().unwrap();
    musician_thread.join().unwrap();

    /*********************************************************
     *
     *  CHANNELS (with numbers and multiple producers)
     *
     *********************************************************/

    // Create a new channel
    // int_tx: variable used by the producer - int_rx: variable used by the consumer
    let (int_tx, int_rx) = mpsc::channel();

    // Clone the receiver to be used from the second consumer!
    let int_tx_two = int_tx.clone();

    // Create one producer thread join handler
    //   - Use 'move' in the closure to transfer the ownership 'int_tx' variable to the thread
    let number_producer_one = thread::spawn(move || {
        for number in 0..5 {
            println!("Producing the {} number from producer 1...", number);
            thread::sleep(Duration::from_millis(100));

            // Send the value to the channel (and transfer the ownership)
            int_tx.send(number).unwrap();
        }
    });

    // Create another producer thread join handler
    //   - Use 'move' in the closure to transfer the ownership 'int_tx' variable to the thread
    let number_producer_two = thread::spawn(move || {
        for number in 5..10 {
            println!("Producing the {} number from producer 2...", number);
            thread::sleep(Duration::from_millis(100));

            // Send the value to the channel (and transfer the ownership)
            int_tx_two.send(number).unwrap();
        }
    });

    // Create the first consumer thread join handler
    //   - Use 'move' in the closure to transfer the ownership of the 'int_rx' variable to the thread
    let number_consumer = thread::spawn(move || {
        for received_number in int_rx {
            println!("Consuming the {} number from consumer", received_number);
        }
    });

    // Execute the threads
    number_producer_one.join().unwrap();
    number_producer_two.join().unwrap();
    number_consumer.join().unwrap();
}
