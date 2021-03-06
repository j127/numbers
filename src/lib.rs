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
    // type inferred from return value of function (Vec<u8>)
    (1..=limit).collect()
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

#[test]
fn generate_sequence_should_work() {
    let result = generate_sequence(3);
    assert_eq!(result, &[1, 2, 3]);
}
