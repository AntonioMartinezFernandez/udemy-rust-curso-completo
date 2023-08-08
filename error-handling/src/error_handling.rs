use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn error_handling() {
    /* PANIC & RESULT */

    // Recoverable error 1
    let file_path = "invalid_file.txt";
    let file = File::open(file_path);

    match file {
        Ok(file) => read_file(file),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                println!("error reading file. error kind: {:?}", err.kind())
            }
            _ => println!("error reading file: {}", err),
        },
    }

    // !Force panic error (non-recoverable error)
    // panic!("explode!")
}

fn read_file(file: File) {
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => println!("{}", l),
            Err(_) => (),
        }
    }
}
