fn largest<T : std::cmp::PartialOrd>(list: &[T]) -> &T { //T implementa el trait PartialOrd, sino no compila porque usa >
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest // retorno implicito
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list); //pasaje por referencia
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list); //pasaje por referencia
    println!("The largest char is {}", result);

}