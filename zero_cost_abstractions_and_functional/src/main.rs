extern crate is_odd;
use is_odd::IsOdd;
fn main() {

    use std::time::Instant;

    let now = Instant::now();
    {
        println!("{}", sum_odd_numbers(1000000000));
    }
    let elapsed = now.elapsed();


    let nowF = Instant::now();
    {
        println!("{}", sum_odd_numbers_func(1000000000));
    }
    let elapsedF = nowF.elapsed();

    
    println!("Procedural time: {:.8?}", elapsed);
    println!("FunctionalTime time: {:.8?}", elapsedF);


}

fn sum_odd_numbers(n: u64) -> u64 {
    //procedural
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
    // zero cost abstractions
    (0..) // lista infinita
        .take_while(|element| element < &n) // guarda/pattern matching
        .filter(|n| n.is_odd())
        .fold(0, |sum, element| sum + element) //funcional
}