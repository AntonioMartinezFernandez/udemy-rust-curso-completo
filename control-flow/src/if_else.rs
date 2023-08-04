pub fn if_else() {
    /* IF - ELSE */
    let number = 31;
    if number > 50 {
        println!("number greater than 50")
    } else if number > 25 {
        println!("number greater than 25 and less than 50")
    } else {
        println!("number less than 25")
    }

    let result = if number >= 25 {
        "number equal or greater than 25"
    } else {
        "number less than 25"
    };
    println!("{}", result);
}
