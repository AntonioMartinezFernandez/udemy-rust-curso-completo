use std::ops::Deref;

pub fn drop_trait() {
    /* DROP
        - This trait said what to do when the instance exit the scope
    */

    let name = "Charles Babbage".to_string();
    let copy = MyBoxStruct::new(name);

    println!("copied inventor: {}", *copy);

    // forced drop (used with concurrency)
    let other_name = "Ada Lovelace";
    let other_copy = MyBoxStruct::new(other_name);
    println!("forced variable drop");
    drop(other_copy);

    {
        let machine = "Analytical Engine".to_string();
        let copy = MyBoxStruct::new(machine);

        println!("copied machine: {}", *copy);
        println!("end internal scope");
    }
    println!("end function scope");
}

struct MyBoxStruct<T>(T);

impl<T> MyBoxStruct<T> {
    fn new(value: T) -> MyBoxStruct<T> {
        MyBoxStruct(value)
    }
}

impl<T> Deref for MyBoxStruct<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBoxStruct<T> {
    fn drop(&mut self) {
        println!("MyBoxStruct has exited the scope!")
    }
}
