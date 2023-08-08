use std::rc::Rc;

pub fn reference_counted() {
    /* REFERENCE COUNTED
        - Allow multiple owners for a value
        - We use RC when we want to assign data in the heap memory to be accessed
          in several parts of the code, and we can't stablish in compilation time
          who will be the last one using this data. If we knew who the last one was, we
          could make him the owner, but we don't know it.
        - RC is a counter of owners of a reference. When there is no owners, the data is removed from memory.
    */

    enum List {
        Node(String, Rc<List>),
        None,
    }

    use List::*;

    // _node0 ->
    //           node2 -> node3 -> none
    // _node1 ->

    let node3 = Node(String::from("node3"), Rc::new(None));
    let node2 = Node(String::from("node2"), Rc::new(node3));

    let node2_rc = Rc::new(node2); // create a reference counter of 'node2'

    let _node1 = Node(String::from("node1"), Rc::clone(&node2_rc));
    let _node0 = Node(String::from("node0"), Rc::clone(&node2_rc));

    println!("number of references: {}", Rc::strong_count(&node2_rc));
}
