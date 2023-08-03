pub fn closures() {
    /* CLOSURES */
    let mut number = 49;

    // Using a function
    let adder = add_one;
    println!("adder result {}", adder(number));

    // Creating a basic closure alternative
    let adder_closure = |n| n + 1;
    // let adder_closure = |n: i32| -> i32 { n + 1 };
    println!("adder_closure result {}", adder_closure(number));

    // Taking a variable by reference (normal behaviour)
    let mut increment_closure = || {
        number += 1;
        println!("increment_closure {}", number)
    };
    increment_closure();

    // Taking a variable by value (forced behaviour with 'move')
    let mut increment_closure_by_value = move || {
        number += 1;
        println!("increment_closure_by_value {}", number);
    };
    increment_closure_by_value();
}

fn add_one(n: i32) -> i32 {
    n + 1
}
