pub fn types() {
    /* TYPES */
    // Integers (i8, u8, i16, u16, i32, u32, i64, u64, i128, u128)
    let integer_eight: i8 = 127;
    let negative_integer_eight: i8 = -127;
    let unsigned_integer_eight: u8 = 255;

    // Integer literals
    let decimal: i16 = 12_345;
    let hex: i16 = 0xff;
    let octal: i16 = 0o77;
    let binary: i16 = 0b1111_0000;

    // Floats (f32, f64)
    let float_thirtytwo: f32 = 5.0;
    let float_sixtyfour: f64 = 1234567890.12345;

    // Booleans
    let trusty: bool = true;
    let falsy: bool = false;

    // Characters
    let character: char = 'a';
    let emoji: char = '👍';

    // Tuples
    let tuple: (char, i32, f64) = ('🎩', 23, 0.1);
    let (tuple_char, tuple_int, tuple_float) = tuple;

    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        integer_eight,
        negative_integer_eight,
        unsigned_integer_eight,
        decimal,
        hex,
        octal,
        binary,
        float_thirtytwo,
        float_sixtyfour,
        trusty,
        falsy,
        character,
        emoji,
        tuple.0,
        tuple.1,
        tuple.2,
        tuple_char,
        tuple_int,
        tuple_float
    );

    // Arrays ([type; elements])
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", my_array[0]);

    // strings
    let static_name: &'static str = "Kurt";
    let string_name: &str;
    string_name = "John";

    println!("{} {}", static_name, string_name);

    // Strings
    let grunge_musician: String = "Kurt Cobain".to_string();
    let mut _jazz_musician: String = String::new();
    _jazz_musician = "John Coltrane".to_string();

    println!("{} {}", grunge_musician, _jazz_musician);
}
