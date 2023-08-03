pub fn show_welcome() {
    println!("func welcome");
}

pub fn add(a: &i64, b: &i64) -> i64 {
    // return *a + *b;
    *a + *b
}

pub fn rest(a: &i64, b: &i64) -> i64 {
    // return *a - *b;
    *a - *b
}

pub fn greeting(name: &str) -> String {
    let mut greeting = "Hello ".to_string();
    greeting = greeting + name;
    greeting = greeting + "!";
    return greeting;
}
