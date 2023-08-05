// We need to import HashSet from 'std' library
use std::collections::HashSet;

pub fn hashsets() {
    /* HASHSETS
        avoid repeated items in a collection
    */

    let mut beatles: HashSet<&String> = HashSet::new();
    let john = "John".to_string();
    let paul = "Paul".to_string();
    let ringo = "Ringo".to_string();
    let george = "George".to_string();

    beatles.insert(&john);
    beatles.insert(&paul);
    beatles.insert(&ringo);
    beatles.insert(&george);
    //repeat (so it's not included)
    beatles.insert(&george);

    let invalid_member = "Mick Jagger".to_string();
    beatles.insert(&invalid_member);
    beatles.remove(&invalid_member);

    for component in beatles {
        println!("beatle name: {}", *component);
    }

    // METHODS
    let mut hash_one: HashSet<u8> = HashSet::new();
    hash_one.insert(1);
    hash_one.insert(2);
    hash_one.insert(3);

    let mut hash_two: HashSet<u8> = HashSet::new();
    hash_two.insert(3);
    hash_two.insert(4);
    hash_two.insert(5);

    // union: unique elements in two sets
    let union = hash_one.union(&hash_two);
    for elem in union {
        println!("union val {}", elem);
    }

    // difference: elements in the first set but not in the second set
    let difference = hash_one.difference(&hash_two);
    for elem in difference {
        println!("difference val {}", elem);
    }

    // intersection: common elements in two sets
    let intersection = hash_one.intersection(&hash_two);
    for elem in intersection {
        println!("intersection val {}", elem);
    }

    // symmetric_difference: elements in one element OR in the other
    let symmetric_difference = hash_one.symmetric_difference(&hash_two);
    for elem in symmetric_difference {
        println!("symetric_difference val {}", elem);
    }
}
