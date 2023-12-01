use std::{fs, collections::HashMap};

use regex::{Regex};



fn count(lines: Vec<String>) -> Option<usize>{
    lines
    .iter()
    .map(|line| {
        let s= line
        .chars()
        .filter(|c| c.is_numeric());
        

        let first: char = s.clone().next().unwrap();
        let last = s.last().unwrap();

        let x = [first, last];
        let y: String = x.iter().collect();
        return y;
   
    }
    )
    .map(|x| x.parse::<usize>().unwrap())
    .reduce(|acc, el| acc + el)
}



fn main(){
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines = binding.lines();


    let re = Regex::new("(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)");
    let re_reverse = Regex::new("(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|1|2|3|4|5|6|7|8|9)");


    let mapping = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);





    let transformed_lines = lines.map(|line| {
        let first = re.as_ref().map(|r| r.find(line)).unwrap().unwrap().as_str();

        let reversed_line: String = line.chars().rev().collect();
        let second: String = re_reverse.as_ref().map(|r|r
            //...non-overlapping... oh no...
            .find(&reversed_line)).unwrap().unwrap().as_str().chars().rev().collect();

        let first_number = mapping.get(first).unwrap_or(&first).clone().to_string();
        let second_number = mapping.get(&second.as_str()).unwrap_or(&second.as_str()).clone().to_string();

        let rr = first_number + &second_number;

        println!("{}", rr);

        return rr;


 }
).collect::<Vec<String>>();
    

    let something = count(transformed_lines).unwrap();
    println!("{}", something);
}
