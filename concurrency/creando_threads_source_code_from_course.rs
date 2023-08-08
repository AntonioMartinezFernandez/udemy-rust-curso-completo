use std::thread;
use std::time::Duration;

// Concurrency = Concurrencia
// Programación concurrente con Rust
// Paralelismo
fn main() {
  
  let nombre = String::from("Julio");
  let nombre_clone = nombre.clone();

  println!("Hola {}", nombre);

  let join_handle = thread::spawn( move || { 
    println!("{} se unió a la partida", nombre_clone);
    thread::sleep(Duration::from_millis(2000));
  });

  join_handle.join().unwrap();

  println!("¿Qué quieres hacer hoy {}?", nombre);
}
