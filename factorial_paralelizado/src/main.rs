use num::{BigUint, Zero, One};
use std::mem::replace;

use parallel_factorial::factorial;


fn fac(n:usize,b: BigUint) -> BigUint {
    if n==0 || n == 1{
        b
    }else{
        fac(n-1, n * b)
    }
}

fn main() {
    println!("fac(500000) = {}", factorial(500000));
}
