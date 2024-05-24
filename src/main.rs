mod myfunc;
mod other_funcs;

use crate::myfunc::add_five;
use crate::other_funcs::minus_funcs::sub_five;

//Everything defaults to immutable

fn main() {
    println!("Hello, world!");
    print!("balls to Monty\n");

    let mut x: u32 = 5;
    println!("X is {}", x);

    let y: u32 = add_five(x);
    println!("Y is {}", y);

    x = 60;

    println!("X is {}", x);

    let z: u32 = sub_five(x);
    println!("Z is {}", z);
}
