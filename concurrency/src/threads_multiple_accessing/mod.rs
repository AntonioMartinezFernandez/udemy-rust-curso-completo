use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn main_threads_multiple_accessing() {
    /* ACCESSING FROM MULTIPLE THREADS TO THE SAME DATA */

    /************************************
     *
     *  MUTEX
     *
     *  1. Create Mutex
     *  2. Method/Thread 1 lock the data
     *  3. Method/Thread 1 modify the data
     *  4. Method/Thread 1 unlock the data
     *  5. Method/Thread 2 lock the data
     *  6. Method/Thread 2 modify the data
     *  7. Method/Thread 2 unlock the data
     *  ...
     *
     ************************************/

    // Create Mutex with a value
    let user_id = Mutex::new(12345);
    println!("initial value of Mutex: {:?}", user_id);

    // Change the Mutex value in a different scope
    {
        let mut number = user_id.lock().unwrap();

        println!("initial locked Mutex value: {}", number);

        *number = 54321;

        println!("modified locked Mutex value: {}", number);
    }

    // Print the last value of the Mutex
    println!("last value of Mutex: {:?}", user_id);

    /************************************
     *
     *  MUTEX (with threads)
     *
     ************************************/

    // Create Mutex with a value
    let cash = Mutex::new(1000);

    // Create an Atomic Reference Counted (Arc) of the Mutex and its clones!
    let cash_arc = Arc::new(cash);

    let cash_arc_one = Arc::clone(&cash_arc);
    let cash_arc_two = Arc::clone(&cash_arc);
    let cash_arc_three = Arc::clone(&cash_arc);

    println!("initial cash Arc Mutex: {:?}", cash_arc);

    // Create thread join handlers
    let thread_one = thread::spawn(move || {
        let mut money = cash_arc_one.lock().unwrap();
        *money += 250;
    });

    let thread_two = thread::spawn(move || {
        let mut money = cash_arc_two.lock().unwrap();
        *money -= 100;
    });

    let thread_three = thread::spawn(move || {
        let mut money = cash_arc_three.lock().unwrap();
        *money += 5;
    });

    // Execute threads
    thread_one.join().unwrap();
    thread_two.join().unwrap();
    thread_three.join().unwrap();

    println!("final cash Arc Mutex: {:?}", cash_arc);
}
