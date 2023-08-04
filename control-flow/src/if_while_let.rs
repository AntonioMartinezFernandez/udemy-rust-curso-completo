pub fn if_while_let() {
    /* IF LET */
    let age: Option<i32> = Some(38);
    // verbose way
    match age {
        Some(value) => println!("age: {}", value),
        None => (), // do nothing
    }
    // better way with 'if let'
    if let Some(value) = age {
        println!("age: {}", value)
    }

    /* WHILE LET */
    let mut messages: Option<i32> = Some(3);
    // verbose way
    loop {
        match messages {
            Some(value) => {
                if value > 0 {
                    println!("available messages");
                    messages = Some(value - 1);
                } else {
                    println!("no messages");
                    messages = None; // IMPORTANT to avoid infinite loop
                }
            }
            None => {
                break; // exit loop
            }
        }
    }
    // better way with 'if while'
    messages = Some(3);
    while let Some(value) = messages {
        if value > 0 {
            println!("available messages");
            messages = Some(value - 1);
        } else {
            println!("no messages");
            messages = None;
        }
    }
}
