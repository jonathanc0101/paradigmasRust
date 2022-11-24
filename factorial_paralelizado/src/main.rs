use num::{BigUint, One}; //numeros de tamaño arbitrario

use parallel_factorial::factorial; //factorial paralelizado


fn fac(n:usize) -> BigUint {

    let mut t = BigUint::one();
    for x in 1..n {
        t*=x;
    }

    t
}

fn fac_rec(n:usize,b:BigUint) ->BigUint{
    if n==0 || n == 1{
        b
    }else{
        fac_rec(n-1, n * b)
    }
    
}

fn main() {

    use std::time::Instant;

    let now = Instant::now();
    {
        println!("fac(1000000) = {}", factorial(1000000)); 
    }
    let elapsed = now.elapsed();

    let now_np = Instant::now();
    {
        println!("fac(100000) = {}", fac(100000)); //notese el cambio en la magnitud
    }
    let elapsed_np = now_np.elapsed();


    println!("Factorial paralelizado de 1000000 en: {:.8?}", elapsed);
    println!("Factorial no paralelizado de 100000 en: {:.8?}", elapsed_np);
}
