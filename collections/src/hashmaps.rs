// We need to import HashMap from 'std' library
use std::collections::HashMap;

pub fn hashmaps() {
    /* HASHMAPS */

    // Create hashmap
    let mut temperatures: HashMap<String, f32> = HashMap::new();
    temperatures.insert(String::from("Monday"), 22.3);
    temperatures.insert(String::from("Tuesday"), 25.6);
    temperatures.insert(String::from("Wednesday"), 31.2);
    temperatures.insert(String::from("Thursday"), 32.3);
    temperatures.insert(String::from("Friday"), 35.6);
    temperatures.insert(String::from("Saturday"), 41.2);

    // Insert a new value only if the key has not been defined before
    let sunday = String::from("Sunday");
    temperatures.entry(String::from("Monday")).or_insert(23.4); // Don't insert anything
    temperatures.entry(String::from("Sunday")).or_insert(42.7); // Insert a new value

    // Obtain value from hashmap key
    let sunday_value: Result<&f32, &str> = match temperatures.get("Sunday") {
        Some(value) => Ok(value),
        None => Err("invalid key"),
    };
    if sunday_value.is_ok() {
        let temp = *sunday_value.unwrap();
        println!("Sunday value: {}", temp);
    }

    // Delete value from hashmap
    temperatures.remove(&sunday);

    // Iterate keys/values of hashmap
    for (key, value) in temperatures {
        println!("key: {} - value: {}", key, value);
    }
}
