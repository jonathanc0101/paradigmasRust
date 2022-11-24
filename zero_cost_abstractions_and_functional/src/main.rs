extern crate is_odd;
use is_odd::IsOdd;

fn main() {
    println!("Hello, world!");
    println!("{}", sum_odd_numbers(100000000));
    println!("{}", sum_odd_numbers_func(100000000));
}

fn sum_odd_numbers(n: u64) -> u64 {
    let mut acc = 0;
    for element in 0.. {
        if element >= n {
            break;
        }
        if element.is_odd() {
            acc += element;
        }
    }
    acc
}

fn sum_odd_numbers_func(n: u64) -> u64 {
    (0..)
        .take_while(|element| element < &n)
        .filter(|n| n.is_odd())
        .fold(0, |sum, element| sum + element)
}