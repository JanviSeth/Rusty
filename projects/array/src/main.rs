use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let last = a[4];

    println!("The first element in the array is of value: {first}");
    println!("The last element in the array is of value: {last}");

    println!("Please enter an array index...");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {index} is: {element}"
    );
}
