/// Function: add_five
/// 
/// # Arguments (num : u32  - 32 bit unsigned integer)
/// 
/// # Returns (u32 - 32 bit unsigned integer)
/// 
/// # Example
/// 
/// ```
/// 
/// let x: u32 = 10;
/// let y: u32 = add_five(x);
/// ```
/**
 
 This is a multiline block and this
 is for the add_five function

*/

pub fn add_five(num: u32) -> u32 {
    /*
        First item
        Returns result

     */
    num + 5
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn adds_five_test() {
        let x: u32 = 100;
        let y: u32 = add_five(x);
        println!("x and y are from test: {} {}", x, y);
        assert!(y == 105);
    }
}
