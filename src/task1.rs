use crate::draw;
use std::str::FromStr;

use draw::Draw;

pub fn solve(input: &str) -> usize {

    let evaluation_target = Draw{
        green: 13,
        red: 12,
        blue: 14
    };

    let mut possible_games_sum: usize = 0;

    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<_>>();
        let game_number: usize = parts[0].get(5..)
            .expect("Malformatted input, cannot extract the game number.")
            .parse()
            .expect("Malformatted input, cannot parse the game number.");

        let draw_strs = parts[1].split(';').collect::<Vec<_>>();
        let draws: Vec<_> = draw_strs.iter().flat_map(|x| Draw::from_str(x)).collect();


        // println!("{:?}", draws);
        let possible = draws.iter().all(|draw| evaluation_target.possible_to_draw(draw));


        if possible {
            possible_games_sum += game_number;
        }
    }

    return possible_games_sum;
}