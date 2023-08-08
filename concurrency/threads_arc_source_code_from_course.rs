use std::sync::{Arc, Mutex};

// Concurrency = Concurrencia
// Programación concurrente con Rust
// Mutexes: permiten el acceso al mismo dato desde distintas hebras, por turnos.
fn main() {
  // Arc: similar a Rc, pero es thread safe, es seguro de usarlo en 
  // situaciones concurrentes.
  // Arc: Atomic Reference counted (smart pointer)
  // atomic: son primitivos seguros de compartir en distintas hebras (threads)

  let id = Arc::new(Mutex::new(99));
  let mut handles = vec![];

  for _ in 0..3 { 
    let num_clone = Arc::clone(&id);
    let handle = std::thread::spawn(move || {
      let mut num = num_clone.lock().unwrap();
      *num += 1; 
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
  println!("{:?}", id);
}
