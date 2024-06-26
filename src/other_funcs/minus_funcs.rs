pub fn sub_five(num: u32) -> u32 {
    num - 5
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn minus_five_test() {
        let x: u32 = 100;
        let y: u32 = sub_five(x);
        println!("x and y are from test: {} {}", x, y);
        assert!(y == 95);
    }
}
