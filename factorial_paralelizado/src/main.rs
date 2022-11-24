use num::{BigUint, Zero, One};
use std::mem::replace;

use parallel_factorial::factorial;

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn fac(n:usize,b: BigUint) -> BigUint {
    if n==0 || n == 1{
        b
    }else{
        fac(n-1, n * b)
    }
}

fn main() {
    // This is a very large number.
    // println!("fib(1000) = {}", fib(100000));
    println!("fac(500000) = {}", factorial(500000));
}
