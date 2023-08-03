pub fn variables() {
    /* VARIABLES */
    let string_variable = "string variable";
    println!("{string_variable}");

    let string_variable_with_type: &str = "string variable";
    println!("{string_variable_with_type}");

    let numeric_variable = 10;
    println!("{numeric_variable}");

    let numeric_variable_with_type: i8 = 20;
    println!("{numeric_variable_with_type}");

    let mut mutable_numeric_variable: i16 = 30;
    println!("{mutable_numeric_variable}");
    mutable_numeric_variable = 40;
    println!("{mutable_numeric_variable}");
}
