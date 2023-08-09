use std::time::Duration;

pub fn async_basics() {
    /* ASYNC */

    // Using Threads
    std::thread::spawn(|| {
        let _content = read_file("foo/bar.txt");
    });

    std::thread::sleep(Duration::from_millis(1000));

    // Using Async
    let _file_content_future = read_file_async("async/foo/bar.txt");
    // ! `await` is only allowed inside `async` functions and blocks only allowed inside `async` functions and blocks
    //let file_content = file_content_future.await;
    // * So, we need to create an Async Runtime
}

// Function with IO bounded task (depends on input/output operations to finish the task)
fn read_file(path: &str) -> String {
    println!("Reading {path} file...");
    std::thread::sleep(Duration::from_millis(500));
    "file content".to_string()
}

// Same function in async mode
async fn read_file_async(path: &str) -> String {
    println!("Reading {path} file...");
    std::thread::sleep(Duration::from_millis(500));
    "file content".to_string()
}
