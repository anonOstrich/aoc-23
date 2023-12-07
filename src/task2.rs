use std::{collections::HashMap, cmp::Ordering};


#[derive(Debug, PartialEq, Eq, Ord)]

struct Hand {
    hand_type: HandType,
    cards: Vec<u8>,
    bet: u32
}

// Doesn't need modifications for jokers
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_diff: i16 = (self.hand_type as i16) - (other.hand_type as i16);
        if type_diff > 0 {
            return Some(Ordering::Greater);
        } else if type_diff < 0 {
            return Some(Ordering::Less);
        }

        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            if c1 < c2 {
                return Some(Ordering::Less);
            }
            if c1 > c2 {
                return Some(Ordering::Greater);
            }
        }
        return Some(Ordering::Equal);
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
#[repr(u8)]

enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

impl HandType {
    fn from_counts(counts: &Vec<u8>) -> Self {
        if counts.contains(&5){     
            return  HandType::FiveOfAKind;
           } else if counts.contains(&4) {
               return HandType::FourOfAKind;
           } else if counts.contains(&3) && counts.contains(&2) {
               return HandType::FullHouse;
           } else if counts.contains(&3) {
               return  HandType::ThreeOfAKind;
           } else if counts.iter().filter(|x| **x == 2).count() == 2 {
               return  HandType::TwoPair;
           } else if counts.contains(&2) {
               return HandType::OnePair;
           } else {
               return HandType::HighCard;
           };
    }
}

fn card_val_to_number(c: &char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '2'..='9' => c.to_digit(10).unwrap() as u8,
        'J' => 1,
        _ => panic!("Unknown card value")
    }
}

impl Hand {

    fn from(hand_str: &str) -> Self{
        let mut hand_iter = hand_str.split(' ');
        let cards_str = hand_iter.next().expect("Could not read the hand from input");
        let bet_str = hand_iter.next().expect("Could not read the bet amount from input");

        let cards: Vec<_> = cards_str.chars().map(|c| card_val_to_number(&c)).collect();
        let bet = bet_str.parse().expect("Could not parse");


        let mut map: HashMap<u8, u8> = HashMap::new();

        for card in &cards {
            if card == &1 {
                continue;
            }

            if !map.contains_key(card) {
                map.insert(*card, 0);
            }
            map.insert(*card, 
                *map.get(&card).unwrap() + 1
            );
        }

        let counts: Vec<u8> = map.iter()
        // Don't consider jokers at this point -- what is the hand like with only the remaining cards?
        .filter(|(k, _)| *k != &1).map(|(_, v)| v)
        .map(|x| *x).collect();

        let mut hand_type = HandType::from_counts(&counts);

        // Best strategy for using jokers is always: 
        // Convert them all to the value that is most common
        // If many values could reach the same number of cards,
        // choose the one with the greatest value
        if cards.contains(&1) {
            
            let nof_jokers: usize = cards.iter().filter(|c| **c == 1).count();
            let mut replacement: u8 = 0;
            let mut max_count = 0;

            for (k, v) in &map {
                if v > &max_count || (v == &max_count && k > &replacement) {
                    max_count = *v;
                    replacement = *k;
                }
            }
            


            let mut new_map = map.clone();
            new_map.insert(replacement, max_count + nof_jokers as u8);
            let new_values: Vec<u8> = new_map.values().map(|x| *x).collect();
            hand_type = HandType::from_counts(&new_values);
        }


        
        
        return Hand { hand_type: hand_type, cards: cards, bet: bet }


    }
}





pub fn solve(input: &str) -> i64 {
    let mut hands: Vec<_> = input.lines().map(|l| Hand::from(l)).collect();
    hands.sort();
    

    let result: usize = hands.iter().enumerate().map(|(idx, hand)| (idx + 1) * (hand.bet as usize))
    .sum();

    return result as i64;
}