// Ownership = propiedad, que es dueño de.
// Borrowing = pedir prestado

// Rust no hay garbage collection
// Cada data tiene 1 dueño (owner)

fn main() {
  // Stack
  // implementada como stack (estructura de datos pila)
  // tamaño fijo
  // rápida, basta con mover el puntero
  // es liberada cuando se alcanza el fin del scope

  // Heap
  // Flexible
  // Costoso de asignar y recuperar datos
  // es liberada cuando no tiene dueños (owners)

  // race conditions
  // threads
  let mut edad = 20;
  aumentar_edad(&mut edad);
  //println!("{}", edad);

  let mut nombre = String::from("Julio");
  enviar_nombre(&mut nombre);
  println!("{}", nombre);
}

fn enviar_nombre(nombre2: &mut String) {
  // Julio -> Julio-20210621
  nombre2.push_str("-20210621");
  println!("enviar: {}", nombre2);
}

fn aumentar_edad(edad_copia: &mut i32) {
  *edad_copia += 1;
} // edad_copia es liberada
