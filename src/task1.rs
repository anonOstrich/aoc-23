
fn pretty_print(g: &Vec<Vec<bool>>) {
    for r in 0..g.len() {
        for c in 0..(g[0].len()) {

                        print!("{}", if g[r][c] {'#'} else {'.'});
        }
        print!("\n");
    }
}

/*
Notes: 
- the number of pairs: N over 2 (order doesn't matter)
- shortest path may pass through other galaxies
- is there a quick trick to solving the distances, if there are no obstacles? all the shortest paths need a minimum of delta x steps horizontally and delta y steps vertically
- 
*/

pub fn solve(input: &str) -> isize {

    // Parse input
    let galaxies: Vec<Vec<bool>> = input.lines().map(|line| line.chars().map(|c| match c {
        '.' => false,
        '#' => true,
        _ => panic!("Not possible")
    }).collect()).collect();


    // Expand the galaxy
    let mut expanded_galaxies : Vec<Vec<bool>> = vec![];
    for r in 0..galaxies.len() {
        
        expanded_galaxies.push(galaxies[r].clone());
        if galaxies[r].iter().all(|spot| !spot) {
            expanded_galaxies.push(galaxies[r].clone());
        }
    }



    let mut expanded2_galaxies: Vec<Vec<bool>> = expanded_galaxies.clone();
    
    let mut indices_to_add: Vec<usize> = vec![];

    // This length will change, though...
    for c in 0..expanded_galaxies[0].len() {
        let mut all_empty = true;
        for r in 0..expanded_galaxies.len() {
            if expanded_galaxies[r][c] {
                all_empty = false;
                break;
            }
        }

        if all_empty {
            indices_to_add.insert(0, c);
        }
        
    }

    for c_idx in indices_to_add {
        for r in 0..expanded_galaxies.len() {
            expanded2_galaxies[r].insert(c_idx, false);
        }
    }


    // Find shortest pairwise distances

    let mut galaxy_positions: Vec<(usize, usize)> = vec![];
    for r in 0..expanded2_galaxies.len() {
        for c in 0..expanded2_galaxies[0].len() {
            if expanded2_galaxies[r][c] {
                galaxy_positions.push((r, c));
            }
        }
    }

    let answer = galaxy_positions.iter()
        .enumerate()
        .flat_map(|(idx, pos)|   galaxy_positions.iter().
            skip(idx  +1)
            .map(|pos2|  (pos.0 as isize - pos2.0 as isize).abs() + (pos.1 as isize - pos2.1 as isize).abs()
            )
        ).sum();

    // reduce distance matrix into a ... sum?
    answer
}