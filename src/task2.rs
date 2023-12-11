static EXPANSION_COEFF: usize = 1000000;

pub fn solve(input: &str) -> usize {

    // Parse input
    let galaxies: Vec<Vec<bool>> = input.lines().map(|line| line.chars().map(|c| match c {
        '.' => false,
        '#' => true,
        _ => panic!("Not possible")
    }).collect()).collect();


    let mut expanded_rows: Vec<usize> = vec![];
    for r in 0..galaxies.len() {
        if galaxies[r].iter().all(|spot| !spot) {
            expanded_rows.push(r);
        }
    }

    let mut expanded_columns: Vec<usize> = vec![];
    
    for c in 0..galaxies[0].len() {
        let mut all_empty = true;
        for r in 0..galaxies.len() {
            if galaxies[r][c] {
                all_empty = false;
                break;
            }
        }

        if all_empty {
            expanded_columns.push(c);
        }
        
    }



    // Find shortest pairwise distances

    let mut galaxy_positions: Vec<(usize, usize)> = vec![];
    for r in 0..galaxies.len() {
        for c in 0..galaxies[0].len() {
            if galaxies[r][c] {
                galaxy_positions.push((r, c));
            }
        }
    }

    let answer = galaxy_positions.iter().enumerate()
    .flat_map(|(idx, pos)|
        galaxy_positions.iter().skip(idx  +1)
        .map(|pos2| {
            let basic_distance = (pos.0 as isize - pos2.0 as isize).abs() as usize + (pos.1 as isize - pos2.1 as isize).abs() as usize;

            let mut subtraction = 0;

            let row_min = pos.0.min(pos2.0);
            let row_max = pos.0.max(pos2.0);

            let galaxies_in_between_rows = expanded_rows.iter().filter(|row| **row < row_max && **row > row_min).count();

            subtraction += galaxies_in_between_rows;

            let cols_min = pos.1.min(pos2.1);
            let cols_max = pos.1.max(pos2.1);

            let galaxies_in_between_cols = expanded_columns.iter().filter(|col| **col < cols_max && **col > cols_min).count();
            subtraction += galaxies_in_between_cols;

            return basic_distance + (galaxies_in_between_rows + galaxies_in_between_cols) * EXPANSION_COEFF - subtraction;
        })
    ).sum();
    answer
}