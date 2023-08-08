/*
  UNIT TESTING

    Asserting methods:
      assert
      assert!(expression)
      assert_eq!(a, b)
      assert_ne!(a,b)


    Run tests from terminal:
    cargo test <test_function_name or test_file>

    Example:
    cargo test unit_tests
*/

#[test]
fn number_str_must_return_true_test() {
    let sut = "12345";

    assert!(only_numbers(sut));
}

#[test]
fn str_with_letters_must_return_false_test() {
    let sut = "Z1234";

    assert!(!only_numbers(sut), "'sut' contains letters");
}

#[test]
fn add_two_and_two_must_return_four_test() {
    let res = add(2, 2);

    assert_eq!(res, 4);
}

#[test]
fn add_two_and_two_must_not_return_five_test() {
    let res = add(2, 2);

    assert_ne!(res, 5);
}

#[test]
fn divide_five_from_two_must_return_two_point_five_test() {
    let res = divide(5, 2);

    assert_eq!(res, 2.5);
}

#[test]
#[should_panic] // must throw panic error
fn divide_from_zero_must_throw_panic_test() {
    divide(2, 0);
}

#[test]
#[ignore] // this test will be ignored
fn ignored_test() {
    assert_eq!("a", "a");
}

/* ###################################
   Some functions just for testing
################################### */
fn only_numbers(s: &str) -> bool {
    s.chars().all(char::is_numeric)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: i32, b: i32) -> f32 {
    if b == 0 {
        panic!("division by zero");
    }
    return a as f32 / b as f32;
}
