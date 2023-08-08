use std::ops::Deref;

pub fn deref_trait() {
    /* DEREF
        - This trait make posible the dereferentiation (*)
    */

    // with references...
    let x = 5;
    let y = &x;
    let z = &y;

    if x == 5 {
        println!("x = 5")
    }

    if *y == 5 {
        println!("y = 5")
    }

    if **z == 5 {
        println!("z = 5")
    }

    // with Box...
    let a = 5;
    let b = Box::new(a);
    let c = Box::new(&b);
    let d = Box::new(*b);

    if a == 5 {
        println!("a = 5")
    }

    // 'Box' implements the Deref trait, so can be dereferenced
    if *b == 5 {
        println!("b = 5")
    }

    if ***c == 5 {
        println!("c = 5")
    }

    if *d == 5 {
        println!("d = 5")
    }

    // with custom implementation...
    let custom_a = 5;
    let custom_b = MyCustomBox::new(custom_a);

    if custom_a == 5 {
        println!("custom_a = 5")
    }

    if *custom_b == 5 {
        println!("custom_b = 5")
    }
}

struct MyCustomBox<T>(T);

impl<T> MyCustomBox<T> {
    fn new(value: T) -> MyCustomBox<T> {
        MyCustomBox(value)
    }
}

impl<T> Deref for MyCustomBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
