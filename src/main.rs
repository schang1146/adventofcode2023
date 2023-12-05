use std::io;

fn main() {
    println!("Advent of Code 2023");

    println!("Input problem to run:");

    let mut problem = String::new();
    io::stdin()
        .read_line(&mut problem)
        .expect("Failed to read problem number");

    println!("Running problem: {problem}...");
}
