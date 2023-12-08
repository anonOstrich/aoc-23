use core::panic;
use std::{collections::{HashMap, HashSet}, cmp::Ordering};

fn parse_line(line: &str) -> (String, (String, String)) {
    let mut iterator = line.split('=').map(|x| x .trim());
    let key = iterator.next().expect("Malformed line input");

    let temp = iterator.next().expect("Malformed line input").chars().skip(1).take_while(|c| *c != ')')
    .filter(|c| *c != ',').collect::<String>();
    let targets: Vec<_> =  temp.split(' ').collect();

    return (key.to_string(), (targets[0].to_string(), targets[1].to_string()));
}

/*
    Let's find the cycle lengths for all the inputs.
    (S, I) where state is some state and I is a character in the input
    |S| = 766, |I| = 281

    So there are 766 * 281 = 215246 different total information states. If we ever end up in a same TI state, we've found a cycle. So we need to check max 215246 transitions to find information about the whole cycle of a starting state.

    We do this for each starting state. Then try to find an index, where all the states are in a final state.


    We could form: 
    - a map that tells how many steps until a final state from each state we encounter. 
    - find the longest cycle. Extend other cycles to match the length. Find the final state set
    - do the above but more mathematically... 

    P.S. The real input didn't need quite as general treatment.
*/

#[derive(Debug)]
struct CycleInfo {
    start_state: String,
    // Although with the longer input there is only one index that is the finish
    finishing_indices: Vec<usize>,
    cycle_start_idx: usize,
    // Elements belonging in the cycle (so 1->2->3->1 has length 3)
    cycle_length: usize
}

#[derive(Eq, PartialEq, Hash, Debug,Clone)]
struct State {
    id: String,
}

fn extract_cycle_info(init_state: &str, directions: &str, transitions: &HashMap<String, (String, String)>) -> CycleInfo{


    let mut visited_ti_states: HashMap<(State, usize), usize> = HashMap::new();
    let mut simple_visited_state: HashSet<State> = HashSet::new();
    let mut finishing_states: Vec<usize> = vec![];


    let char_array: Vec<char> = directions.chars().collect();
    let mut ti_state: (State, usize) = (State{id: init_state.to_string()}, 0);
    let mut global_idx = 0;

    let mut saw_final_state = false;
    let mut final_count = 0;

    // Will end eventually if the problem is solvable
    for (idx, direction) in char_array.iter().enumerate().cycle() {



        ti_state.1 = idx;

        if (&ti_state.0).id.ends_with('Z') {
            finishing_states.push(global_idx);
        }

        if visited_ti_states.contains_key(&ti_state) {
            let cycle_len = global_idx - visited_ti_states.get(&ti_state).unwrap();
            return CycleInfo {
                cycle_start_idx:  idx, 
                cycle_length: cycle_len,
                finishing_indices: finishing_states,
                start_state: init_state.to_string()
            };
        } 
        visited_ti_states.insert(ti_state.clone(), global_idx);
        simple_visited_state.insert(ti_state.clone().0);


        let options = transitions.get(&ti_state.0.id).unwrap();
        let next_state = match direction {
            'L' => State{id: options.0.to_string()},
            'R' => State{id: options.1.to_string()},
            _ => panic!("Impossible")
        };

        ti_state.0 = next_state; 
        global_idx += 1;
    }

    panic!("impossible");
}



#[derive(Debug)]
struct Equation {
    b: usize,
    k: usize
}

impl Equation {
    fn result(&self, input: usize) -> usize {
        self.b + (self.k * input)
    }

    fn max_below(&self, limit: usize, earlier_limit: usize) -> (usize, usize) {
        let mut coeff = earlier_limit;
        let mut res = self.result(coeff);

        while res <= limit {
            coeff += 1;
            res = self.result(coeff);
        }
        return (self.result(coeff - 1), coeff - 1)
    }
}

fn easy_solve(paths: &mut Vec<CycleInfo>) -> usize{
    // Make use of the fact that there are is only one final state per track...
    assert!(paths.iter().all(|x| x.finishing_indices.len() == 1));

    paths.sort_by(|e1, e2| match (e1.cycle_length as isize - e1.cycle_start_idx as isize - e2.cycle_length as isize + e2.cycle_start_idx as isize) {
        x if x > 0 => Ordering::Less,
        x if x < 0 => Ordering::Greater,
        _ => Ordering::Equal
    });


    let equations: Vec<_> = paths.iter().map(|info| Equation{b: info.finishing_indices[0], k: info.cycle_start_idx + info.cycle_length - info.finishing_indices[0] + info.finishing_indices[0] - info.cycle_start_idx // info.cycle_length
     }).collect();


    let first_el = &equations[0];

    let mut coeff = 
    0;
    let mut floors: Vec<usize> = equations.iter().skip(1).map(|_| 0).collect();

    while true {
        let path_len = first_el.result(coeff);

        let max_lengths_below: Vec<_> = equations.iter().skip(1).enumerate()
            .map(|(i, x)| x.max_below(path_len, floors[i]))
            .collect();

        if max_lengths_below.iter().all(|(res, _)| *res == path_len) {
            return path_len;
        }         
        max_lengths_below.iter().enumerate().for_each(|(i, (_, cff))| floors[i] = *cff);
        coeff += 1;
    }

    panic!("Impossible");
}

pub fn solve(input: &str) -> usize{
    let mut iterator = input.lines();
    let navigation_instructions = iterator.next().expect("Input does not have first line");

    let state_mappings: HashMap<String, (String, String)> = iterator.skip(1).map(|line| parse_line(line)).collect(); 



    let current_states: HashSet<String> = input.lines().skip(2).map(|x| x[..3].to_string()).filter(|x| x.ends_with('A')).collect();

    let mut cycles: Vec<_> = current_states.iter()
        .map(|state| extract_cycle_info(state, navigation_instructions, &state_mappings)).collect();
    return easy_solve(&mut cycles);
}