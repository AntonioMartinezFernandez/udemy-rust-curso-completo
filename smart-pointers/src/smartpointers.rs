pub fn smartpointers() {
    /* SMARTPOINTERS

    Smartpointers:
    - Like references, but are owners of the memory, are capable of modify it, have metadata and additional features
    - Usually implemented using structs, but implementing Deref and Drop traits
    - Deref allow smart pointer instances to behave as references (to allow use it with code that works with references)
    - Drop allow to define logic executable when the smart pointer exit the scope
     */

    /* BOX */
    let x = 2; // saved in the stack memory
    let y = Box::new(2); // saved in the heap memory

    println!("x from stack: {} - y from heap (BOX): {}", x, y);

    // Example with linked list
    enum List {
        Node(i32, Box<List>),
        None,
    }

    let node3 = List::Node(30, Box::new(List::None));
    let node2 = List::Node(20, Box::new(node3));
    let node1 = List::Node(10, Box::new(node2));

    match node1 {
        List::Node(num, _next) => println!("node1 value: {num}"),
        List::None => (),
    }
}
