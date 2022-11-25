fn main() {
    let mut s = String::from("hello");

    let r0 = &s; // que pasa aca
    let r1 = &mut s; // que pasa aca
    let r2 = &mut s; // que pasa aca

    println!("{}, {}", r1, r2);

}
