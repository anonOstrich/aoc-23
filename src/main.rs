mod task1;
mod task2;
mod draw;

use std::fs;

fn main(){
    let input = fs::read_to_string("input.txt").expect("Failed to read the puzzle input.");
    let answer = task2::solve(&input);
    println!("The answer: {answer}");
}