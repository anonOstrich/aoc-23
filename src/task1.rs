use std::collections::HashSet;

#[derive(Debug)]
struct ScratchCard {
    winning: Vec<i32>,
    observed: Vec<i32>
}

impl ScratchCard {
    pub fn from_strs(winning: &str, observed: &str) -> Self {
        ScratchCard{
            winning: parse(winning),
            observed: parse(observed)
        }
    }

    pub fn intersection(&self) -> HashSet<i32> {
        let set1: HashSet<i32> = HashSet::from_iter(self.winning.clone());
        let set2 = HashSet::from_iter(self.observed.clone());

        return set1.intersection(&set2).map(|x| *x).collect();
    }


    // For task 1
    pub fn score(&self) -> i32 {
        let intersection_size = self.intersection().len();

        match intersection_size {
            0 => 0,
            n => (2 as i32).pow((n - 1) as u32)
        }
    }

}

fn parse(input: &str) -> Vec<i32> {
    input.trim().split(' ').flat_map(|x| x.parse::<i32>()).collect::<Vec<_>>()
}


pub fn solve(input: &str) -> i32 {
    input.lines()
        .filter_map(|line| line.split(':').last())
        .map(|line|{
            let parts = line.split('|');
            return parts.collect::<Vec<_>>();
        })
        .map(|card_strs| ScratchCard::from_strs(
            card_strs.get(0).expect("Cannot extract cards"),
            card_strs.get(1).expect("Cannot extract cards")
        ))
        .map(|x|x.score())
        .sum()

}