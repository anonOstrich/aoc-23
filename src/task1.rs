use core::fmt;
use std::{ str::{FromStr}, fmt::{Error}};


#[cfg(windows)]
static LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
static LINE_ENDING: &str = "\n";

struct RangeMapping {
    source: i64,
    target: i64,
    range: i64
}

impl RangeMapping {
    fn map(&self, input: i64) -> Option<i64> {

        let delta = input - self.source;
        match delta {
            x if x >= self.range => None,
            x if x >= 0 => Some(self.target + delta),
            _ => None
        }
    }
}

impl FromStr for RangeMapping {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<i64> = s.split(' ')
        .map(|x| x.parse().expect("Failed to parse mapping data as number")).collect();
        if numbers.len() != 3 {
            //...don't really understand error types well yet
            return Err(fmt::Error);
        }

        Ok(RangeMapping {
            target: numbers[0],
            source: numbers[1],
            range: numbers[2]
        })
    }
}

enum Mapping {
    RangeMapping(RangeMapping),
    DefaultMapping
}


impl Mapping {
    fn map(&self, input: i64) -> Option<i64> {
        match self {
            Self::RangeMapping(m) => m.map(input),
            Self::DefaultMapping => Some(input)
        }
    }
}


struct Converter {
    mappings: Vec<Mapping>
}

impl Converter {
    fn from(lines: &str) -> Self {
        let mut mappings: Vec<Mapping> = lines.lines()
            .map(|line| line.parse().expect("Failed to parse a number line"))
            .map(|x| Mapping::RangeMapping(x))
            .collect();

        mappings.push(Mapping::DefaultMapping);
        Converter{ mappings: mappings }
    }

    fn find_output(&self, input: i64) -> i64 {
        let mut something: i64 = input;

        for m in &self.mappings {
            if let Some(n) = m.map(something) {
                return n;
            }
        }
        return input;
    }
}


pub fn solve(input: &str) -> i64 {
    let separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let another_separator = format!(":{}", LINE_ENDING);

    let mut  mappings = input.split(separator.as_str());
    let first_line = mappings.next().expect("Could not read the first (seeds) line");

    let seeds: Vec<i64> = first_line.split(':').last().expect("First row is malformed").trim().split(' ')
    .map(|x| x.parse().expect("Could not parse seed number")).collect();

    let converters: Vec<_> = mappings.map(|mapping_data| {
        let something = mapping_data.split(another_separator.as_str()).last().expect("Failed to parse mapping entry");
        return Converter::from(something);
    }).collect();


   let positions: Vec<_> =  seeds.iter().map(|seed| converters.iter().fold(*seed, |acc: i64, el: &Converter|{
        let next = el.find_output(acc);
        return next;
   } )).collect();
    
    return *positions.iter().min().expect("Something went wrong -- no positions to choose from");
}