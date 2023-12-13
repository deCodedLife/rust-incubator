extern crate basics;

#[cfg(test)]
mod tests {

    fn nani() -> i8 {
        5
    }

    #[test]
    fn sum_numbers() {
        basics::test2("Some data");
        assert_eq(2, nani())
    }
}
