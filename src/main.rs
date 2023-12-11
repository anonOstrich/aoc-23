mod task1;
mod task2;

use std::fs;

fn main(){
    let file_name = "test-input.txt";
    let input = fs::read_to_string(file_name).expect("Failed to read the puzzle input.");

    let answer = task2::solve(&input);
    println!("Answer: {answer}");
}