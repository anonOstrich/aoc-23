use std::collections::{HashMap, HashSet};



pub fn solve(input: &str) -> usize {
    let input_matrix: Vec<_> = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect();
    let height = input_matrix.len();
    let width = input_matrix.get(0).expect("There should be at least one line").len();

    dbg!(height);
    dbg!(width);

    let mut num_matrix: Vec<Vec<usize>> = Vec::new();
    let mut filter_matrix: Vec<Vec<bool>> = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(false);
        }
        filter_matrix.push(row);
    }

    let mut idx_to_value_map: HashMap<usize, usize> = HashMap::new();

    let mut num_id = 1;
    let mut collected_num_str = String::new();
    let mut continues_number = false;

    // Construct map and matrix of the numbers in the schematics
    for r in 0..height {
        let mut num_row : Vec<usize> = Vec::new();
        if continues_number {
            // Add possible ending thing as number
            let n = collected_num_str.parse().expect("Could not parse number string");

            idx_to_value_map.insert(num_id, n);

            collected_num_str.clear();
            continues_number = false;
            num_id += 1;
        }

        continues_number = false;
        for c in 0..width {
            let char = input_matrix[r][c];

            if char.is_digit(10) {
                
                continues_number = true;
                collected_num_str.push(char);
                num_row.push(num_id);

            } else if continues_number {
                let n = collected_num_str.parse().expect("Could not parse number string");

                idx_to_value_map.insert(num_id, n);
  
                collected_num_str.clear();
                continues_number = false;
                num_id += 1;
                num_row.push(0);

            } else {
                num_row.push(0);
            }
        }
        num_matrix.push(num_row);
    }

    // if the input ends in a number
    if continues_number {
        // Add possible ending thing as number
        let n = collected_num_str.parse().expect("Could not parse number string");

        idx_to_value_map.insert(num_id, n);
    }


    // Construct matrix of positions with characters
    for r in 0..height {
        for c in 0..width {
            let char = input_matrix[r][c];

            match char {
                '.' => (),
                '0'..='9' => (),
                _ => {
                        for dy in -1..=1 {
                            let row: isize = r as isize + dy;
                        
                            if row < 0 || row > (height as isize) - 1 {
                                continue;
                            }

                            for dx in -1..=1 {
                                let col: isize = c as isize + dx;
                                if col < 0 || col > (width as isize) - 1 {
                                    continue;
                                }
                                filter_matrix[row as usize][col as usize] = true;
                            }
                        }
                }
            };
            
            

        }
    }


    // Ids of the numbers that should be included in the sum
    let mut idx_set: HashSet<usize> = HashSet::new();

    for r in 0..height {
        for c in 0..width {
            if filter_matrix[r][c] && num_matrix[r][c] > 0 {
                idx_set.insert(num_matrix[r][c]);
            }
        }
    }


    let answer: usize = idx_set.iter().filter_map(|id| idx_to_value_map.get(id)).sum();
    return answer;
}