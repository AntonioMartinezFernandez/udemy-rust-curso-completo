pub fn ownership() {
    /* OWNERSHIP
        Ownership: variables have owners
        Borrowing: this ownership can be borrowed

        - There is no garbage collector in Rust
        - Any 'data' have ONLY ONE owner
        - This behaviour is very useful when we work with concurrency and parallelism, because its avoid race conditions

        Stack vs. Heap:
            STACK:
                - Stack memory is implemented like a stack data structure
                - Fixed size
                - Fast, just move the pointer
                - Is released when the scope ends

            HEAP:
                - Flexible
                - Is more costly to get and set data
                - Is released when it is no longer owned

    */

    /* ##### Example 1 ##### */
    let original_num: i8 = 24;
    let copy_num = original_num;

    /*
    VALID USE:
     - The numeric variables clone their value when we assign its value to another variable
       (are located in the stack memory because we know their size), so we can use both variables
    */
    println!("original_num: {}", original_num);
    println!("copy_num: {}", copy_num);

    /* ##### Example 2 ##### */
    let original: String = String::from("my value");
    let copy = original;

    /*
    INVALID USE:
     - The 'String' variables are created in the heap memory.
     - The 'original' variable can't be used because the ownership of the data has been transfered to the 'copy' variable
    */
    // println!("original : {}", original);

    println!("copy: {}", copy);

    /* ##### Example 3 ##### */
    let original: String = String::from("value to be cloned");
    let copy = original.clone();

    /*
    VALID USE:
     - The 'original' variable can be used because the data has been cloned to the 'copy' variable,
       then we have two independents variables located in the heap memory.
    */
    println!("original : {}", original);
    println!("copy: {}", copy);

    /* ##### Example 4 ##### */
    let age_by_value: i32 = 49;
    increase_age_by_value(age_by_value);
    println!("age after increase_age_by_value: {}", age_by_value);

    /* ##### Example 5 ##### */
    let mut age_by_reference: i32 = 51;
    increase_age_by_reference(&mut age_by_reference);
    println!("age after increase_age_by_reference: {}", age_by_reference);

    /* ##### Example 6 ##### */
    let guitar: String = String::from("Stratocaster");
    send_guitar(&guitar);
    println!("guitar: {}", guitar);

    /* ##### Example 7 ##### */
    let mut bass: String = String::from("Pastorius");
    println!("unsigned bass: {}", bass);
    sign_bass(&mut bass);
    println!("signed bass: {}", bass);

    /* ##### Example 8 ##### */
    let message: &str = "this is a string literal!";
    print_message(message);
}

fn increase_age_by_value(mut _received_age_by_value: i32) {
    _received_age_by_value += 1;
    // '_received_age_by_value' is a new variable created in the stack memory.
    // This function doesn't return any value, and when the scope ends, the created variables
    // are removed from the stack memory.
}

fn increase_age_by_reference(received_age_by_reference: &mut i32) {
    let new_age = *received_age_by_reference + 1;
    *received_age_by_reference = new_age;
    // 'new_age' is a new variable created in the stack memory.
    // This function doesn't return any value, and when the scope ends, the created variables
    // are removed from the stack memory.
    // 'received_age_by_reference' is the reference to the mutable variable 'age_by_reference'.
    // We can change the value in memory of this variable thanks to the Borrowing concept (the ownership
    // of the 'age_by_reference' variable data is borrowed to the 'received_age_by_reference' variable).
    // When the scope of this function ends, the Ownership of this data in the stack memory
    // is transferred back to the 'age_by_reference' variable.
}

fn send_guitar(guitar: &String) {
    // 'String' variables are located in the heap memory, so, to be able to return the ownership to the original
    // variable, we pass the reference of the variable to this function, then we can use the data inside
    // the function, and later, when the scope ends, return the ownership to the original variable.
    println!("sending guitar {}", *guitar)
}

fn sign_bass(bass: &mut String) {
    bass.push_str(" SIGNED!");
}

fn print_message(msg: &str) {
    let msg_string: String = String::from(msg);
    println!("message: {}", msg_string);
}
