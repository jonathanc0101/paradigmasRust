fn main() {
    let s1 = String::from("hello"); // move es un shallow copy
    let s2 = s1; // ahora solo s2 es valida
    // let s2 = s1.clone(); // cambiamos las cosas, y ahi si anda

    println!("{}, world!", s1); // error, previene aliasing
    // println!("{}, world!", s2); // no hay error
}

