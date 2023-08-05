pub fn strings() {
    /* STRINGS
    String vs string slice:
        string:
            - UTF8 char collection
            - Saved in 'heap' memory (can change his size)
        string slice:
            - Reference to contiguous sequence of collection elements
            - Saved in 'stack' memory (is a reference)
            - Embedded in the binary code
            - Immutables
    */

    // String
    let string = String::from("My string");
    println!("string {}", string);

    // String slice
    let string_slice = "My string slice";

    let mut string_from_slice = string_slice.to_string();

    string_from_slice.push(' ');
    string_from_slice.push('X');

    let slice_from_string = &string_from_slice[..string_from_slice.len()];
    println!(
        "slice created from string (previously created as slice and modified): {}",
        slice_from_string
    );
}
