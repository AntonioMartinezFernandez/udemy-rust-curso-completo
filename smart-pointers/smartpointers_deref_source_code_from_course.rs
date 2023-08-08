use std::ops::Deref;

// Smart pointers = punteros inteligentes
fn main() {
  // Defer trait: hacer posible la dereferenciación (*)
  let x = 5;
  let y = MiCaja::new(x);

  if x == 5 {
    println!("hola");
  }
  if *y == 5 {
    println!("hola");
  }
}

struct MiCaja<T>(T);

impl<T> MiCaja<T> {
  fn new(x:T) -> MiCaja<T> {
    MiCaja(x)
  }
}

impl<T> Deref for MiCaja<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target { 
    &self.0
  }
}
