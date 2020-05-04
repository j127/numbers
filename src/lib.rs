pub fn say_hello() {
    println!("hello world");
}

pub fn print() {
    // alternate syntax for arrays
    // let numbers = [1u8, 2, 3, 4, 5];
    // let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    // let numbers = [1, 2, 3, 4, 5];

    // using Vec<T> instead of an array
    let numbers = vec![1, 2, 3, 4, 5];

    // a hack to view the type by causing a compiler error
    // let () = numbers;

    // use `.iter()` for an array, but it isn't required for vec
    for n in numbers {
        println!("{}", n);
    }
}
