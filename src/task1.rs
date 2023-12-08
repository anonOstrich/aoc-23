use std::collections::HashMap;

fn parse_line(line: &str) -> (String, (String, String)) {
    let mut iterator = line.split('=').map(|x| x .trim());
    let key = iterator.next().expect("Malformed line input");

    let temp = iterator.next().expect("Malformed line input").chars().skip(1).take_while(|c| *c != ')')
    .filter(|c| *c != ',').collect::<String>();
    let targets: Vec<_> =  temp.split(' ').collect();

    return (key.to_string(), (targets[0].to_string(), targets[1].to_string()));
}




pub fn solve(input: &str) -> i64 {
    let mut iterator = input.lines();
    let navigation_instructions = iterator.next().expect("Input does not have first line");

    let state_mappings: HashMap<String, (String, String)> = iterator.skip(1).map(|line| parse_line(line)).collect(); 


    let mut current_state = "AAA";
    let final_state = "ZZZ";
    let mut steps = 0;
    for dir in navigation_instructions.chars().cycle() {
        if current_state == final_state {
            break;
        }
        steps += 1;
        let x = state_mappings.get(current_state);

        let next_state = match dir {
            'L' => &state_mappings[current_state].0,
            'R' => &state_mappings[current_state].1,
            _ => panic!("Direction that is not L or R should be impossible")
        };
        current_state = next_state;
    }
    
    return steps;
}