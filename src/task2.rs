use std::{str::FromStr, fmt::Error};


#[derive(Debug)]
struct Sequence {
    numbers: Vec<Vec<isize>>
}

impl FromStr for Sequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<isize> = s.split_whitespace().map(|x| x.parse().expect("Could not parse a value as number"))
            // The only addition to task 2
            .rev()
            .collect();
        return Ok(Sequence {numbers: vec![nums]});
    }
}

impl Sequence {
    fn solve_next(&mut self) -> isize {
        let mut idx = 0;

        while true {
            let prev = &self.numbers[idx];
            let next: Vec<_> = prev.iter().zip(
                    prev.iter().skip(1)
                )
                .map(|(prev, next)| next - prev).collect();

            idx += 1;

            if next.iter().all(|x| *x == 0) {
                self.numbers.push(next);
                break;
            }
            self.numbers.push(next);

        }


        let mut next_value = 0;

        while idx > 0 {
            let shallower = &self.numbers[idx - 1];

            next_value += shallower.last().unwrap();
            idx -= 1;
        }

       return next_value;
    }
}

pub fn solve(input: &str) -> isize {
    let mut sequences: Vec<Sequence> = input.lines().map(|x| x.parse().unwrap()).collect();
    let predictions: isize = sequences.iter_mut().map(|s| s.solve_next()).sum();

    predictions
}