// Some example macros
macro_rules! three {
    () => {
        2 + 1
    };
}

macro_rules! add_three {
    ($x:expr) => {
        $x + 3
    };
}

macro_rules! create_vector {
    ($($x:expr),*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
}

macro_rules! new_function {
    ($name:ident) => {
        fn $name() {
            println!("Hi! I'm {}()", stringify!($name));
        }
    };
}

fn main() {
    /******************************************************
     *    MACROS
     *
     *    - Code which write code (meta-programming)
     *    - Useful for simplify expressions
     *    - Can be used for DSL (Domain Specific Languages)
     *    - Can implement functionality in Runtime
     *
     *
     ******************************************************/

    let macro_three_result = three!();
    println!("{macro_three_result}");

    let macro_add_three_result = add_three!(3);
    println!("{macro_add_three_result}");

    let macro_create_vector = create_vector!("how", "are", "you");
    println!("{:?}", macro_create_vector);

    new_function!(greeting);
    greeting();
}
