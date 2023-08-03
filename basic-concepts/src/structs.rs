pub fn structs() {
    /* STRUCTS */
    let mut user1 = User {
        name: "Marcus Miller".to_string(),
        email: "marcus@miller.com".to_string(),
        age: 70,
        active: true,
    };

    user1.age = 80;

    println!(
        "{} {} {} {}",
        user1.name, user1.email, user1.age, user1.active
    );

    let user2 = new_user(
        String::from("Charlie Parker"),
        String::from("charlie@parker.com"),
        92,
        false,
    );

    println!(
        "{} {} {} {}",
        user2.name, user2.email, user2.age, user2.active
    );

    // spread
    let user3 = User {
        name: "Thelonious Monk".to_string(),
        email: "thelonious@monk.com".to_string(),
        ..user2
    };

    println!(
        "{} {} {} {}",
        user3.name, user3.email, user3.age, user3.active
    );

    // struct method
    let birth = user1.birth_year();
    println!("birth {}", birth);

    // tuple structs
    let point = Point(15, 20, 25);
    println!("{} {} {}", point.0, point.1, point.2);
}

// Struct
struct User {
    name: String,
    email: String,
    age: i32,
    active: bool,
}

// Struct method
impl User {
    fn birth_year(&self) -> i32 {
        let actual = 2023;
        return actual - self.age;
    }
}

// Tuple struct
struct Point(i32, i32, i32);

// Init shorthand
fn new_user(name: String, email: String, age: i32, active: bool) -> User {
    // User {
    //     name: name,
    //     email: email,
    //     age: age,
    //     active: active,
    // }
    User {
        name,
        email,
        age,
        active,
    }
}
