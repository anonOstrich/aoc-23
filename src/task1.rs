use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, PartialEq, Eq, Ord)]

struct Hand {
    hand_type: HandType,
    cards: Vec<u8>,
    bet: u32
}

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

fn card_val_to_number(c: &char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '2'..='9' => c.to_digit(10).unwrap() as u8,
        _ => panic!("Unknown card value")
    }
}

impl Hand {

    fn from(hand_str: &str) -> Self{
        let mut hand_iter = hand_str.split(' ');
        let cards_str = hand_iter.next().expect("Could not read the hand from input");
        let bet_str = hand_iter.next().expect("Could not read the bet amount from input");

        let cards = cards_str.chars().map(|c| card_val_to_number(&c)).collect();
        let bet = bet_str.parse().expect("Could not parse");


        let mut map: HashMap<u8, u8> = HashMap::new();

        for card in &cards {
            if !map.contains_key(card) {
                map.insert(*card, 0);
            }
            map.insert(*card, 
                *map.get(&card).unwrap() + 1
            );
        }

        let counts: Vec<u8> = map.values().map(|x| *x).collect();

        let mut hand_type = HandType::HighCard;
        if counts.contains(&5){
             
         hand_type =  HandType::FiveOfAKind;
        } else if counts.contains(&4) {
            hand_type = HandType::FourOfAKind;
        } else if counts.contains(&3) && counts.contains(&2) {
            hand_type = HandType::FullHouse;
        } else if counts.contains(&3) {
            hand_type =  HandType::ThreeOfAKind;
        } else if counts.iter().filter(|x| **x == 2).count() == 2 {
            hand_type =  HandType::TwoPair;
        } else if counts.contains(&2) {
            hand_type = HandType::OnePair;
        } else {
            hand_type = HandType::HighCard;
        };
        
        return Hand { hand_type: hand_type, cards: cards, bet: bet }


    }
}

pub fn solve(input: &str) -> i64 {
    
    let mut hands: Vec<_> = input.lines().map(|l| Hand::from(l)).collect();
    hands.sort();
    

    let result: usize = hands.iter().enumerate().map(|(idx, hand)| (idx + 1) * (hand.bet as usize)).sum();

    return result as i64;
}