pub fn say_hello() {
    println!("hello world");
}

// type inference doesn't operate on function signatures
fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    // Creating an empty vector doesn't allocate until you put something
    // in it.
    let mut numbers = Vec::new();
    for n in 1..=limit {
        numbers.push(n)
    }
    numbers
}

pub fn print(limit: u8) {
    let vector_numbers = generate_sequence(limit);
    // no "move" error, because it passes slices
    output_sequence(&vector_numbers);
    output_sequence(&vector_numbers);

    println!("=======");

    let array_numbers = [1, 2, 3, 4, 5];
    output_sequence(&array_numbers);
    output_sequence(&array_numbers);
}
