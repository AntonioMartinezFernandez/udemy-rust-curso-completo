pub fn let_else() {
    /* LET ELSE */

    // let some_number: Option<i32> = None;
    let some_number: Option<i32> = Some(100);

    // old way
    match some_number {
        Some(num) => println!("num: {num}"),
        None => println!("invalid num"),
    }

    // alternative way
    if let Some(num) = some_number {
        println!("num: {num}")
    } else {
        println!("invalid num")
    }

    // new way with let else (from Rust v1.65)
    // this way help us with errors control.
    let Some(num) = some_number else{ // divergent actions (manage error with return/panic/break)
      panic!("invalid num");
    };
    println!("num: {num}"); // apply logic to the validated existent value

    // Use case example
    let operations_result = do_operations(some_number);
    if operations_result.is_err() {
        println!(
            "operations result error {}",
            operations_result.err().unwrap()
        );
    }
    if operations_result.is_ok() {
        println!("operations result {}", operations_result.unwrap());
    }
}

// Function that make use of LET ELSE
fn do_operations(input: Option<i32>) -> Result<i32, &'static str> {
    let Some(number) = input else{
    return Err("invalid number");
  };

    let result = number * 2 + 50 - 10;
    return Ok(result);
}
