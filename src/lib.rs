pub fn say_hello() {
    println!("hello world");
}

pub fn print() {
    // alternate syntaxes
    // let numbers = [1u8, 2, 3, 4, 5];
    // let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    let numbers = [1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n);
    }
}
