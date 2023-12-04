use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct ScratchCard {
    id: i32,
    // There was really no need to store all the numbers, just the relevant
    // value would have sufficed. Expected to use in part 2 before it unlocked
    winning: Vec<i32>,
    observed: Vec<i32>
}



impl ScratchCard {
    pub fn from_str(input: &str) -> Self {

        let mut parts = input.split(':');
        let first_part = parts.next().expect("Could not split the scratchcard");
        let second_part = parts.next().expect("Could not split the scratchcard");

        
        let game_id: i32 = first_part
            .split(' ')
            // Could be more than one space to align items
            .skip_while(|x| x.is_empty())
            .last().expect("malformatted string before :")
            .parse().expect("failed to parse game id");

        let mut number_sets =  second_part.split('|').map(|card_str| parse(card_str));

        ScratchCard{
            id: game_id,
            winning: number_sets.nth(0).unwrap(),
            observed:number_sets.last().unwrap()
        }
    }

    pub fn intersection(&self) -> HashSet<i32> {
        let set1: HashSet<i32> = HashSet::from_iter(self.winning.clone());
        let set2 = HashSet::from_iter(self.observed.clone());

        return set1.intersection(&set2).map(|x| *x).collect();
    }

    pub fn matching_number(&self) -> i32 {
        self.intersection().len() as i32
    }

}

fn parse(input: &str) -> Vec<i32> {
    input.trim().split(' ').flat_map(|x| x.parse::<i32>()).collect::<Vec<_>>()
}


pub fn solve(input: &str) -> i32 {
    let mut counts : HashMap<i32, i32> = HashMap::new();
    // The original scratch cards
    for idx in 1..=input.lines().count() {
        counts.insert(idx as i32, 1);
    }

    input.lines()
        .map(|line| ScratchCard::from_str(line))
        .for_each(|card| {
            let match_num = card.matching_number();

            for i in (card.id + 1)..=(card.id + match_num) {
                let previous_count = *counts.get(&i).unwrap();
                let current_cards = *counts.get(&card.id).unwrap();

                counts.insert(i, previous_count + current_cards);
            }
        });

    let result = counts.values().sum();
    return result;

}