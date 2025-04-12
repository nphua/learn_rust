use std::io;

fn main() {
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    another_function(index);
}

fn another_function(index: usize) {
    let a = [1, 2, 3, 4, 5];

    if index < a.len() {
        let element = a[index];
        println!("The value of the element at index {} is {}", index, element);
    } else {
        println!("Index {} is out of bounds!", index);
    }
}
