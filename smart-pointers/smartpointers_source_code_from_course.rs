// Smart pointers = punteros inteligentes
fn main() {
  // referencias (&)
  // Reference Counter: contador de referencias
  // String y Vec<T>

  // Smart pointers son usualmente implementados usando structs, 
  // pero implementando los traits Deref y Drop.
  // Deref permite a las instancias de smart pointer comportarse como referencias,
  // para que el mismo código que funciona con referencias, funcione con smart pointers.
  // Drop trait permite definir lógica que se ejecute una vez que 
  // el smart pointer sale del scope.
  
  // Box<T> = caja
  // heap vs stack
  let x = 2;
  let y = Box::new(2);
  println!("y = {}", y);

  // Linked lists = Listas enlazadas
  // (valor, nodo1) -> (valor2, nodo2) -> (valor3, null)

  enum List {
    Node(i32, Box<List>),
    None,
  }
  
  use List::*;

  let node3 = Node(10, Box::new(None));
  let node2 = Node(3, Box::new(node3));
  let node1 = Node(90, Box::new(node2));

}
