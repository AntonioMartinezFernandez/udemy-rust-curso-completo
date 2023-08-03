pub fn iterators() {
    /* ITERATORS */

    // Trait Iterator with slices
    let slice = [1, 2, 3];

    for val in slice.iter() {
        println!("slice value {}", val);
    }

    // Trait Iterator with vectors
    let mut vector: Vec<String> = Vec::new();
    vector.push("one".to_string());
    vector.push("two".to_string());

    for val in vector.iter() {
        println!("vector value {}", val)
    }

    // Custom Iterator
    let mut counter = Counter::new();
    counter.next();
    let c = counter.next();

    match c {
        Some(count) => println!("c value {}", count),
        None => (),
    }
    println!("counter value {}", counter.count);

    let sum: i32 = counter.sum();
    println!("counter sum value {}", sum);
}

// Custom iterator
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 100 {
            self.count += 1;
            return Some(self.count);
        } else {
            return None;
        }
    }
}
