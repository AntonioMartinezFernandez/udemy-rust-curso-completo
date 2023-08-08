use std::{thread, time::Duration};

pub fn main_threads() {
    /* THREADS (at Operating System level) */
    println!("hi from main thread");

    /************************************
     *
     *  THREADS BASICS
     *
     ************************************/

    // Create a new thread join handler that takes 2 seconds to complete execution
    let first_thread = thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        println!("hi from thread one");
    });

    // Create a new thread join handler that takes 1 second to complete execution

    let second_thread = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("hi from thread two");
    });

    // Execute and unwrap join thread join handlers
    first_thread.join().unwrap();
    second_thread.join().unwrap();

    // Create a simple thread that is executed inmediatly
    //    - Its result will never be showed if we don't wait the end of the thread execution from the main thread
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        println!("hi from simple thread");
    });

    // Wait 1 second to wait the simple thread execution
    thread::sleep(Duration::from_secs(1));

    /************************************
     *
     *  THREADS ACCESSING VARIABLES
     *
     ************************************/

    // We need to create two variables, one for the main thread, and another for the secondary thread
    let name = String::from("Matt Bellamy");
    let name_clone = name.clone();

    println!("say hello to {} from main thread", name);

    // Use 'move' in the closure of the thread to transfer the ownership of the used variables to the new thread
    let greeting_thread = thread::spawn(move || {
        println!("say hello to {} from greeting thread", name);
        thread::sleep(Duration::from_millis(150));
    });

    // Execute the thread (it will use the 'name' variable)
    greeting_thread.join().unwrap();

    // Do something with the 'name' clone variable in the main thread
    println!("say hello one more time from main thread to {}", name_clone);
}
