pub mod closures;
pub mod constants;
pub mod defaults;
pub mod enums;
pub mod generics;
pub mod iterators;
pub mod prelude;
pub mod structs;
pub mod traits;
pub mod types;
pub mod variables;

fn main() {
    variables::variables();
    constants::constants();
    types::types();
    structs::structs();
    enums::enums();
    generics::generics();
    traits::traits();
    defaults::defaults();
    iterators::iterators();
    closures::closures();
    prelude::prelude();

    // Functions
    basic_concepts::custom_functions::functions::show_welcome();

    let add = basic_concepts::custom_functions::functions::add(&123, &456);
    let rest: i64 = basic_concepts::custom_functions::functions::rest(&123, &456);
    println!("add {} - rest {}", add, rest);

    let person = "Esperanza Spalding";
    let greeting = basic_concepts::custom_functions::functions::greeting(person);
    println!("{greeting}");

    // Expressions
    let expression = {
        let x = 4;
        let y = 6;
        x + y
    };
    println!("{expression}")
}
