use crate::draw;
use std::str::FromStr;

use draw::Draw;

pub fn solve(input: &str) -> usize {
    let mut cum_power: usize = 0;

    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<_>>();
        let draw_strs = parts[1].split(';');
        let mut draws = draw_strs.flat_map(|x| Draw::from_str(x));
        let minimum_state = Draw::create_minimal_from(&mut draws);
        cum_power += minimum_state.power();
    }

    return cum_power;
}