use std::{str::FromStr, fmt::Error};
use regex::Regex;
use once_cell::sync::Lazy;



#[derive(Debug)]
pub struct Draw {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Draw {
    fn from(list: &[(usize, &str)]) -> Self {
        let mut draw = Draw {red: 0, green: 0, blue: 0};
        for (n, color) in list {
            match *color {
                "red" => draw.red = *n,
                "green" => draw.green = *n,
                "blue" => draw.blue = *n,
                _ => panic!("The input is malformed -- unknown color."),
            }
        }
        return draw;
    }

    // Only needed for task 1
    pub fn possible_to_draw(&self, obs_draw: &Draw) -> bool {
        self.blue >= obs_draw.blue && self.red >= obs_draw.red && self.green >= obs_draw.green
    }

    // Only needed for task 2

    pub fn create_minimal_from(draws: &mut dyn Iterator<Item=Draw>) -> Self {
        let mut draw = Draw {red: 0, green: 0, blue: 0};
        for d in draws {
            if d.red > draw.red {
                draw.red = d.red;
            }
            if d.green > draw.green {
                draw.green = d.green;
            }
            if d.blue > draw.blue {
                draw.blue = d.blue;
            }
        }
        return draw;
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}


static RE_STR: &'static str = r"(?:(\d+) (red|green|blue)(?: ,)?)+";
static RE: Lazy<Regex> = Lazy::new(|| Regex::from_str(RE_STR).expect("Failed to compile the regex"));

impl FromStr for Draw {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {


        let parsed_colors: Vec<_> = RE.captures_iter(s)
            .map(|c | c.extract())
            .map(|(_, [n, color])| (n.parse::<usize>()
                .expect("The input is malformed -- cannot parse the number of balls."),
                color)
            )
            .collect();
        
        let draw = Draw::from(&parsed_colors);
        return Ok(draw);
    }
}