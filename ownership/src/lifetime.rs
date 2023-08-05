const VALUE: i32 = 256;

pub fn lifetime() {
    /* LIFETIME

      Lifetime: lifetime of the references to memory. Is a way to stay sure that a memory slice is valid for a reference.
        - Only applies to references
        - The longest lifetime is that of the program itself.
        - A reference that has a lifetime as long as the program is called 'static'.
        - All constants are 'static'
        - String literals are 'static'

    */

    do_something(&VALUE, "string_slice");

    // Example 1

    let a: i32; // declare a variable
    {
        let b: i32 = 10; // declare 'b' variable in a different scope
        a = b; // assign 'b' value and ownership to 'a'
    }
    println!("a: {}", a);

    // If we try to do that with references we have an error "`d` does not live long enough"
    // let c: &i32;
    // {
    //     let d: i32 = 10;
    //     c = &d;
    // } // 'd' is freed from memory, so the reference to 'd' does not exist when this scope ends
    // println!("c: {}", c)

    {
        let f: i32 = 10;
        let g: i32 = 11;
        let h: &i32 = giveme_higher_ref(&f, &g);
        println!("higher ref: {}", h)
    }

    {
        let string: String = String::from("my_string");
        let new_string_literal: &str = giveme_ownership(&string);
        println!("string: {}", string);
        println!("new_string_literal: {}", new_string_literal);
    }
}

fn do_something<'a, 'b>(param_a: &'a i32, param_b: &'b str) -> &'a i32 {
    println!("received {}", param_b);
    param_a
}

fn giveme_higher_ref<'a, 'b>(param_a: &'a i32, param_b: &'a i32) -> &'a i32 {
    if param_a > param_b {
        return param_a;
    } else {
        return param_b;
    }
}

fn giveme_ownership<'a>(param: &'a String) -> &'a str {
    &param
}
