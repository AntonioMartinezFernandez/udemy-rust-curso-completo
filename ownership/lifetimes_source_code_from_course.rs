const VALOR: i32 = 222;

fn main() {
// Lifetimes = tiempo de vida, de las referencias a memoria
// Lifetimes son una forma de asegurar que un pedazo 
// de memoria es aún valida para una referencia.
  let a: i32;
  {
    let b = 10;
    a = b;

  }// b es liberada
  //println!("{}", a);

  let b = dame_ref(&VALOR, &a);
  println!("{}", b);

  let hola = dame_ownership(String::from("hola"));
  let name: &'static str = "Julio"; // literal o harcoded, con lifetime static
}

fn dame_ownership<'a>(param: String) -> &'a str {
  &param // esto hace que no compile!
}

fn dame_ref<'a, 'b>(param_a: &'a i32, param_b: &'a i32) -> &'a i32 {
  if param_a > param_b {
    param_a
  } else {
    param_b
  }
}


fn hace_algo<'a>(param: &'a i32, param_b: &'a i32) -> i32 {
  3
}

fn hace_algo2(param: i32, param_b: String) -> i32 {
  3
}
