pub fn traits() {
    /* TRAITS */

    // Using custom structs
    let clark = Human {};
    let molly = Cat {};

    println!("human hello: {}", clark.say_hello());
    println!("cat hello: {}", molly.say_hello());

    println!("common human lang: {}", Human::language());
    println!("common cat lang: {}", Cat::language());

    // Using existent struct
    let age: Option<i32> = Some(17);
    if age.authorized() {
        println!("authorized to drive a car")
    } else {
        println!("not authorized to drive a car")
    }

    // Implement custom std::fmt::Debug and custom std::fmt::Display methods for a struct
    let nagwrok = Alien {
        name: "Nagwrok".to_string(),
        planet: "Saturn".to_string(),
    };
    println!("{:?}", nagwrok);
    println!("{}", nagwrok);

    // Derive
    let martian = Martian {
        _name: "ET".to_string(),
    };
    println!("{:?}", martian);
}

// Trait for custom struct
trait Speak {
    fn say_hello(&self) -> String;
    fn language() -> String;
}

struct Human;

impl Speak for Human {
    fn say_hello(&self) -> String {
        return "Hi friends!".to_string();
    }

    fn language() -> String {
        return "Human lang".to_string();
    }
}

struct Cat;

impl Speak for Cat {
    fn say_hello(&self) -> String {
        return "Meow!".to_string();
    }

    fn language() -> String {
        return "Cat lang".to_string();
    }
}

// Trait for existent struct
trait Drive {
    fn authorized(&self) -> bool;
}

impl Drive for Option<i32> {
    fn authorized(&self) -> bool {
        match self {
            Some(age) => age > &18,
            None => false,
        }
    }
}

// Custom std::fmt::Debug method implementation and requisites
struct Alien {
    name: String,
    planet: String,
}

impl std::fmt::Debug for Alien {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Alien {} is from {}", self.name, self.planet)
    }
}

// Custom std::fmt::Display method implementation and requisites
impl std::fmt::Display for Alien {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Alien called {} from planet {}", self.name, self.planet)
    }
}

// Using std::fmt::Debug method with Derive
#[derive(Debug)]
struct Martian {
    _name: String,
}
