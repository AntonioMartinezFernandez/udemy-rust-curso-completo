pub fn vectors() {
    /* VECTORS */
    // normal way to create a vector
    let _vector_one: Vec<i32> = Vec::new();

    // create with macro
    let mut vector_two = vec![1, 2];

    // insert data
    vector_two.push(3);

    // access to vector position
    let val = vector_two[1];
    println!("vector_two position 1: {val}");
    // ! trying to access to an invalid position, ends with crash
    // e.g.: let val = vector_two[100]

    // safe way to access to vector position
    match vector_two.get(5) {
        Some(val) => println!("vector_two safe way: {}", val),
        None => (),
    };

    // iterate a vector
    for elem in &vector_two {
        println!("elem {}", elem);
    }

    // iterate and modify values of a vector (like 'map')
    for elem in &mut vector_two {
        *elem = *elem * 2;
    }
    // then values has changed
    for elem in &vector_two {
        println!("modified elem {}", elem);
    }

    // vector of enums
    let messages = vec![Message::TEXT("my message".to_string()), Message::ERROR(404)];
    for message in &messages {
        match message {
            Message::TEXT(msg) => println!("message: {msg}"),
            Message::ERROR(err) => println!("error: {err}"),
        }
    }

    // ! When the scope ends, the vector is automatically removed from memory
}

enum Message {
    TEXT(String),
    ERROR(i32),
}
