#![crate_type = "lib"]
#![crate_name = "testlib"]

pub fn test_func() -> String {
    internal();
    "Hello world from library".to_string()
}

pub fn add_numbers(a: i8, b: i8) -> i16 {
    (a + b) as i16
}

fn internal() {
    println!("to do")
}

#[test]
fn check_numbers_adding() {
    assert_eq!(10, add_numbers(7, 3));
    assert_eq!(10, add_numbers(5, 5));
    assert_eq!(22, add_numbers(21, 1));
    assert_eq!(15, add_numbers(12, 3));
}
