use super::*;

#[test]
fn sum_numbers() {
    test2("datas");
    assert_eq!(18, testlib::add_numbers(10, 8));
}
