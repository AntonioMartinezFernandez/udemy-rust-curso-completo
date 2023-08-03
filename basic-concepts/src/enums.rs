pub fn enums() {
    /* ENUMS */
    struct Musician {
        name: String,
        instrument: Instrument,
        website: Website,
    }

    let musician = Musician {
        name: "Abe Rabade".to_string(),
        instrument: Instrument::PIANO,
        website: Website::URL(String::from("www.aberabade.com")),
    };

    println!(
        "{} {:?} {:?}",
        musician.name, musician.instrument, musician.website
    );

    // Example of enum usecases
    let piano = is_strings_instrument(Instrument::PIANO);
    let saxo = is_strings_instrument(Instrument::SAXO);
    println!("is strings strument? piano-{} saxo-{}", piano, saxo);

    website_is_url(Website::URL("myurl.com".to_string()));
    website_is_url(Website::LINKEDIN("linkedin.com".to_string()));

    // Option enum
    let mut optional_value: Option<String> = None;
    match optional_value {
        Some(optional_value) => println!("value is: {}", optional_value),
        None => println!("{}", "value is null"),
    }

    optional_value = Some(String::from("this is the value"));
    match optional_value {
        Some(optional_value) => println!("value is: {}", optional_value),
        None => println!("{}", "value is null"),
    }

    // Option enum in struct
    struct RockMusician {
        name: Option<String>,
    }
    impl RockMusician {
        fn print_name(&self) {
            if self.name.is_none() {
                println!("name is null");
            } else {
                let musician_name = self.name.clone();
                println!("name is {}", musician_name.unwrap())
            }
        }
    }

    let mut new_musician = RockMusician { name: None };
    new_musician.print_name();

    new_musician.name = Some("Jim Morrison".to_string());
    new_musician.print_name();
}

// Enums
#[derive(Debug)] // mandatory to print the 'enum' value with println
enum Instrument {
    PIANO,
    SAXO,
}

#[derive(Debug)] // mandatory to print the 'enum' value with println
enum Website {
    URL(String),
    LINKEDIN(String),
}

// Example functions for enums
fn is_strings_instrument(instrument: Instrument) -> bool {
    match instrument {
        Instrument::PIANO => true,
        Instrument::SAXO => false,
    }
}

fn website_is_url(website: Website) {
    let is_url: bool;
    match website {
        Website::URL(_url) => is_url = true,
        Website::LINKEDIN(_url) => is_url = false,
    }
    if is_url {
        println!("website is URL");
    } else {
        println!("website not is URL");
    }
}

// Option enum interface
// enum Option<T> {
//     Some(T),
//     None,
// }
