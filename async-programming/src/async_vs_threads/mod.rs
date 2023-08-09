pub fn async_vs_threads() {
    let threads_msg = String::from(
        "OS threads usecases: small number of tasks, CPU-bounded, big computational cost tasks, ...",
    );
    let async_msg =
        String::from("Async usecases: huge number of IO-bounded tasks, servers, databases, ...");

    println!("{threads_msg}");
    println!("{async_msg}");
}
