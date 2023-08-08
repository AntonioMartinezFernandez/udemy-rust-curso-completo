use std::rc::Rc;

fn main() {
  // Reference Counted Smart Pointer: permite que un valor tenga muchos dueños
  // Usamos Rc cuando queremos asignar datos en el heap para que sea accedido
  // en multiples partes del código, y no podemos determinar en tiempo de 
  // compilación el último que accederá estos datos. Si supieramos de antemano
  //  quien sería el último, podríamos hacer que ese último sea el dueño, 
  // pero no lo sabemos. Entonces Rc lleva un contador de referencias con 
  // todos los dueños, y cuando ya no quedan más dueños, puede limpiar el dato.

  enum List {
    Node(i32, Rc<List>),
    None,
  }
  
  use List::*;

  // node1 ->
  //          node2-> node3-> none
  // node0 ->

  let node3 = Node(10, Rc::new(None));
  let node2 = Node(3, Rc::new(node3));
  
  let node2_rc = Rc::new(node2);
  {
    println!("nro de referencias: {}", Rc::strong_count(&node2_rc));
    let node1 = Node(90, Rc::clone(&node2_rc));
    println!("nro de referencias: {}", Rc::strong_count(&node2_rc));
    let node0 = Node(5, Rc::clone(&node2_rc));
  }
  println!("nro de referencias: {}", Rc::strong_count(&node2_rc));

}
