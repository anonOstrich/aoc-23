use core::panic;
use std::{collections::HashSet, fmt::Write, thread::current, process::Output};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    N,
    W,
    S,
    E
}

#[derive(Debug, Copy, Clone)]
struct PipePart {
    hole_positions: (Dir, Dir)
}

#[derive(Debug)]
enum SqType {
    Empty,
    Start,
    Pipe(PipePart)
}

impl SqType {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => Self::Empty,
            'S' => Self::Start,
            other => Self::Pipe(PipePart::from_char(other))
        }
    }
}

impl PipePart {
    fn from_char(c: &char) -> Self {
        let holes = match c {
            '|' => (Dir::S, Dir::N),
            '-' => (Dir::E, Dir::W),
            'J' => (Dir::W, Dir::N),
            '7' => (Dir::W, Dir::S),
            'L' => (Dir::N, Dir::E),
            'F' => (Dir::S, Dir::E),
            other => panic!("Should not be possible: {}", other)
        };

        Self{ hole_positions: holes}
    }

    fn from_dirs(dirs: (Dir, Dir)) -> Self{
        Self{hole_positions: dirs}
    }

    fn next_pipe(&self, last_exit: Dir, (row_idx, col_idx): (usize, usize), map: &Vec<Vec<SqType>>) -> ((usize, usize), Dir) {
        let entry_dir = match last_exit {
            Dir::E => Dir::W,
            Dir::W => Dir::E,
            Dir::S => Dir::N,
            Dir::N => Dir::S
        };
        
        let mut idx = 0;

        if self.hole_positions.0 != entry_dir && self.hole_positions.1 != entry_dir {

            panic!("Something is severely wrong...");
        }

        let exit_dir = if self.hole_positions.0 == entry_dir {
            &self.hole_positions.1
        } else {
            &self.hole_positions.0
        };

        let next_row = row_idx as isize + match exit_dir {
            Dir::N => -1,
            Dir::S => 1,
            _ => 0 
        };

        let next_col = col_idx as isize + match exit_dir {
            Dir::W => -1,
            Dir::E => 1,
            _ => 0
        };

        return ((next_row as usize, next_col as usize), *exit_dir);
    }

    fn hole_to_east(&self) -> bool {
        match self.hole_positions {
            (Dir::E, _) => true,
            (_, Dir::E) => true,
            _ => false
        }
    }

    fn hole_to_west(&self) -> bool {
        match self.hole_positions {
            (Dir::W, _) => true,
            (_, Dir::W) => true,
            _ => false
        }
    }
    fn hole_to_north(&self) -> bool {
        match self.hole_positions {
            (Dir::N, _) => true,
            (_, Dir::N) => true,
            _ => false
        }
    }
    fn hole_to_south(&self) -> bool {
        match self.hole_positions {
            (Dir::S, _) => true,
            (_, Dir::S) => true,
            _ => false
        }
    }

    fn is_corner(&self) -> bool {
        match self.hole_positions {
            (Dir::N, Dir::E) => true,
            (Dir::E, Dir::N) => true,
            (Dir::N, Dir::W) => true,
            (Dir::W, Dir::N) => true,
            (Dir::S, Dir::E) => true,
            (Dir::E, Dir::S) => true,
            (Dir::S, Dir::W) => true,
            (Dir::W, Dir::S) => true,

            _ => false
        }
    }
}


fn find_start_idx(map: &Vec<Vec<SqType>>) -> (usize, usize) {
    let (mut start_row, mut start_col): (usize, usize) = (0, 0);

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            let s = &map[r][c];
            match s {
                SqType::Start =>  {
                    start_row = r;
                    start_col = c;
                    return (r, c);
                },
                _ => {
                    start_row = r;
                    start_col = c;
                }
            }
        }
    }
    panic!("Start not found :/");
    
}

#[derive(Clone, Copy)]
enum OutputSymbol {
    Out,
    In,
    Path
}


struct SolutionVisualization {
    data: Vec<Vec<OutputSymbol>>
}

impl std::fmt::Display for SolutionVisualization{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in &self.data {
             d.iter().map(|e: &OutputSymbol| match e {
                OutputSymbol::In => 'i',
                OutputSymbol::Out => 'o',
                OutputSymbol::Path => 'X'
            }).for_each(|e| {f.write_char(e);});
            f.write_char('\n');
        }
        Ok(())
        //f.write_char('\n')
    }
}


pub fn solve(input: &str) -> usize {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().map(|c| SqType::from_char(&c)).collect()).collect();


    let (start_row, start_col) = find_start_idx(&map);
    

    let mut hole_directions: [Dir; 2] = [Dir::S, Dir::S];
    let mut hole_idx = 0;    
    
    if start_row > 0 {
        match &map[start_row - 1][start_col] {
            SqType::Pipe(n) => {
                if n.hole_positions.0 == Dir::S || n.hole_positions.1 == Dir::S {
                
                    hole_directions[hole_idx] = Dir::N;
                    hole_idx += 1
                }
            },
            _ =>{}
        }
    }

    if start_row < map.len() - 1 {
        match &map[start_row + 1][start_col] {
            SqType::Pipe(n) => {
                if n.hole_positions.0 == Dir::N || n.hole_positions.1 == Dir::N {
                    
    
                    hole_directions[hole_idx] = Dir::S;
                    hole_idx += 1
                }
            },
            _ =>{}
        }
    }
  

    if start_col > 0 {
        match &map[start_row][start_col - 1] {
            SqType::Pipe(n) => {
                if n.hole_positions.0 == Dir::E || n.hole_positions.1 == Dir::E {
                    

                    hole_directions[hole_idx] = Dir::W;
                    hole_idx += 1
                }
            },
            _ =>{}
        }
    }

    if start_col < map[0].len() - 1 {
        match &map[start_row][start_col + 1] {
            SqType::Pipe(n) => {
                if n.hole_positions.0 == Dir::W || n.hole_positions.1 == Dir::W {
                    
                    hole_directions[hole_idx] = Dir::E;
                    hole_idx += 1
                }
            },
            _ =>{}
        }
    }




    



    let start_part = PipePart::from_dirs((hole_directions[0], hole_directions[1]));
    
    let mut path_len = 0;
    let (mut current_r, mut current_c) = (start_row, start_col);
    let mut direction = hole_directions[0];
    let mut current_sq: &SqType = &SqType::Pipe(start_part);


    let first_dir: (isize, isize) =  match hole_directions[0] {
        Dir::E => (0, 1),
        Dir::W => (0, -1),
        Dir::N => (-1, 0),
        Dir::S => (1, 0)
     };

    let mut current_sq = &map[(start_row as isize + first_dir.0) as usize][(start_col as isize  + first_dir.1) as usize];
    path_len += 1;

    current_r = (start_row as isize + first_dir.0) as usize;
    current_c = (start_col as isize + first_dir.1) as usize;

    let mut nice_output : Vec<Vec<OutputSymbol>> = vec![];
    for r in 0..map.len() {
        nice_output.push(vec![]);
        for c in 0..map[0].len() {
            nice_output[r].push(OutputSymbol::Out);
        }
    }
    // Starting symbol, too
    nice_output[start_row][start_col] = OutputSymbol::Path;

    while true {
        nice_output[current_r][current_c] = OutputSymbol::Path;
    
        //dbg!(&current_sq);
        let pipe = match current_sq {
            SqType::Pipe(x) => x,
            _ => panic!("Should not be possible")
        };

        let ((next_row, next_col), exit_dir) = pipe.next_pipe(direction, (current_r, current_c), &map);
        current_sq = &map[next_row][next_col];
        direction = exit_dir;
        current_r = next_row;
        current_c = next_col;
        match &current_sq {
            SqType::Start => {
                break;
            },
            _ => {}
        };
        path_len += 1;
    }




    let mut in_count = 0;

    for r in 0..map.len() {
        let mut currently_in = false;
        let mut continuous_right_line = false;
        for c in 0..map[0].len() {
            let something = &map[r][c];


            match something {
                SqType::Empty => {
                    nice_output[r][c] = if currently_in {
                        in_count+=1;
                       OutputSymbol::In
                    }  else  {
                         OutputSymbol::Out
                    }

                },
                SqType::Start => {
                    if !continuous_right_line&& !start_part.is_corner() {
                        currently_in = !currently_in;
                    } 

                    // Absolutely do nothing!
                    if start_part.hole_to_east() && start_part.hole_to_west() {
                        continue;
                    }

                    if start_part.is_corner() {
                        currently_in = !currently_in;
                    
                    }
                    /* 
                    if continuous_right_line && !n.hole_to_east(){
                        currently_in = !currently_in;
                    }*/
                    continuous_right_line =  start_part.hole_to_east();
                },
                SqType::Pipe(n) => {
                    if !continuous_right_line&& !n.is_corner() {
                        println!("This should only match |: {:?}", n);
                        currently_in = !currently_in;
                    } 

                    // Absolutely do nothing!
                    if n.hole_to_east() && n.hole_to_west() {
                        println!("This should only match -: {:?}", n);
                        continue;
                    }

                    if n.is_corner() {
                        if !n.hole_to_east() {
                            currently_in = !currently_in;
                        }
                        println!("This should only match corners: {:?}", n);
                    }
                    /* 
                    if continuous_right_line && !n.hole_to_east(){
                        currently_in = !currently_in;
                    }*/
                    continuous_right_line =  n.hole_to_east();
                }
            }
        }
    }
    
    for c in 0..map[0].len() {
        let mut currently_in = false;
        let mut continues_down = false;
        for r in 0..map.len() {
            let elem = &map[r][c];

            match &elem {
                SqType::Empty => {
                    nice_output[r][c] = if currently_in {
                        // OutputSymbol::In
                        nice_output[r][c]
                    } else {
                        OutputSymbol::Out
                    }
                },
                SqType::Start => {
                    // THIS IS CERTAINLY CORRECT!
                    if !continues_down && !start_part.is_corner() {
                        currently_in = !currently_in;
                    } 

                    // Absolutely do nothing!
                    // ALSO quite sure...
                    if start_part.hole_to_north() && start_part.hole_to_south() {
                        continue;
                    }

                    if start_part.is_corner() {
                            currently_in = !currently_in;
                    }
                    /* 
                    if continuous_right_line && !n.hole_to_east(){
                        currently_in = !currently_in;
                    }*/
                    continues_down = start_part.hole_to_south();
                },
                SqType::Pipe(n) => {
                   // THIS IS CERTAINLY CORRECT!               
                   if !continues_down && !n.is_corner() {
                    currently_in = !currently_in;
                  } 
                
                // Absolutely do nothing!
                // ALSO quite sure...
                if n.hole_to_north() && n.hole_to_south() {
                    continue;
                }
                
                if n.is_corner() {
                    currently_in = !currently_in;
                }

                    continues_down = n.hole_to_south();
                }
            }

        }
    }
    let res: usize =  nice_output.iter().map(|line| line.iter().filter(|symb| match symb {
        OutputSymbol::In => true,
        _ => false
     }).count()).sum();
    testing(&SolutionVisualization { data: nice_output });



    return res;
}

fn testing(vis: &SolutionVisualization) {
    println!("{vis}");
}