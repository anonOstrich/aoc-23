use core::panic;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    N,
    W,
    S,
    E
}

#[derive(Debug)]
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

    while true {
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
                println!("OH YEAH");
                break;
            },
            _ => {}
        };
        path_len += 1;
    }

    

    if path_len % 2 == 1 {
        return  (path_len / 2) + 1;
    } else {
        return path_len / 2;
    }

    dbg!(path_len);
    1
}