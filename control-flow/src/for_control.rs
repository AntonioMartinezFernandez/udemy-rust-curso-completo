pub fn for_control() {
    /* FOR */
    let my_array = [1, 2, 3];
    for element in my_array.iter() {
        println!("'for' element {}", element)
    }

    for element in (0..4).rev() {
        println!("'for' element of a collection in reverse order {}", element)
    }
}
