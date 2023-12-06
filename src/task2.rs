use core::{fmt, panic};
use std::{str::{FromStr, Lines}, fmt::{Error}};


#[cfg(windows)]
static LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
static LINE_ENDING: &str = "\n";


#[derive(Debug, Clone, Copy, PartialEq)]
enum Range {
    Range(i64, i64),
    Empty
}

impl Range {
    fn intersection(&self, other: &Self) -> Self {
        let something = match (self, other) {
            (Range::Empty, _) => Range::Empty,
            (_, Range::Empty) => Range::Empty,
            (Range::Range(s1, w1),Range::Range(s2, w2)) => {
                let e1 = s1 + w1;
                let e2 = s2 + w2;

                let start = std::cmp::max(s1, s2);
                let end = std::cmp::min(e1, e2);
                if *start >= end {
                    return Range::Empty;
                }
                return Range::Range(*start, end - start);
            }
        };
        return something;
    }

    fn subtract(&self, other: &Self) -> Vec<Self> {
        let inter = self.intersection(other);
        if inter == Self::Empty {
            return vec![*self];
        }
        if *self == Self::Empty {
            return vec![];
        }

        let Self::Range (s1, w1)= *self else {panic!()};
        let Self::Range(s2, w2) = inter else {panic!()};

        if s2 == s1 {
            if w1 == w2 {
                return vec![Self::Empty];
            }
            return vec![Self::Range(s2 + w2, w1 - w2)];
        }

        if s1 + w1 == s2 + w2 {
            return vec![Self::Range(s1, s2 - s1)]
        }


        // need to create two elements for the result...
        return vec![
            Self::Range(s1,s2 - s1),
            Self::Range(s2 + w2, s1 + w1 - (s2 + w2))
        ];
    }

    
}

struct Mapping {
    source: i64,
    target: i64,
    range: i64
}

impl Mapping {

    fn map(&self, input: &Range) -> (Range, Range) {
        // these would make much more sense as properties of mapping
        let source_range = Range::Range(self.source, self.range);
        let target_range = Range::Range(self.target, self.range);

        let source_intersection = input.intersection(&source_range);
        if  Range::Empty == source_intersection {
            return (Range::Empty, Range::Empty);
        }
        let Range::Range(x, y ) = source_intersection else {panic!("Oh no")};
        let target_unbounded = Range::Range(self.target + (x - self.source), y);
        let target = target_unbounded.intersection(&target_range);
        return (target, source_intersection);
    }
}

impl FromStr for Mapping {
    type Err = Error;

    // Expects a string of type "\d \d \d", fails otherwise
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<i64> = s.split(' ')
        .map(|x| x.parse().expect("Failed to parse mapping data as number")).collect();
        if numbers.len() != 3 {
            //...don't really understand error types well yet
            return Err(fmt::Error);
        }

        Ok(Mapping {
            target: numbers[0],
            source: numbers[1],
            range: numbers[2]
        })
    }
}



struct Converter {
    mappings: Vec<Mapping>
}

impl Converter {
    fn from(lines: &str) -> Self {
        let mut mappings: Vec<Mapping> = lines.lines()
            .map(|line| line.parse().expect("Failed to parse a number line"))
            .collect();

        Converter{ mappings: mappings }
    }

    fn find_output(&self, input: &Range) -> Vec<Range> {
        let mut outputs: Vec<Range> = vec![];
        // unused numbers of the original seed range
        let mut unused: Vec<Range> = vec![*input];
        for m in &self.mappings {
            let (output_range, intersected_source_range) = m.map(&input);
            if output_range != Range::Empty {
                outputs.push(output_range);
            }
            unused = unused.iter().flat_map(|range| range.subtract(&intersected_source_range))
            .filter(|x| *x != Range::Empty)
            .collect();
        }

        if unused.len() > 0 {
            // the identity mapping, if none of the listed mappings apply
            outputs.append(&mut unused);
        }

        return outputs;
    }

}


pub fn solve(input: &str) -> i64 {


    let separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let another_separator = format!(":{}", LINE_ENDING);


    let mut  mappings = input.split(separator.as_str());


    let first_line = mappings.next().expect("Could not read the first (seeds) line");
    let mut temp = first_line.split(':').last().expect("First row is malformed").trim().split(' ');

    let seed_ranges: Vec<(i64, i64)> = temp.clone()
        .enumerate()
        .filter(|(i, _)|  i % 2 == 0)
        .zip(
            temp.
            enumerate()
            .filter(|(i, _)| i % 2 == 1)
        )
        .map(|((i1, e1), (i2, e2))| (e1.parse().expect("could not parse seed initial"), e2.parse().expect("could not parse seed initial")))
        .collect();

    let seeds: Vec<_> = seed_ranges.iter().map(|(s, w)| Range::Range(*s, *w)).collect();

    let converters: Vec<_> = mappings.clone().map(|mapping_data| {
        let something = mapping_data.split(another_separator.as_str()).last().expect("Failed to parse mapping entry");
        // dbg!(something);
        return Converter::from(something);
    }).collect();

    let mut current_vals = seeds;

    for conv in converters {
        current_vals = current_vals.iter().map(|input_range| conv.find_output(input_range)).flatten().collect();
    }

    return current_vals.iter().fold(i64::MAX, |min, el| {
        let Range::Range(s, _) = el else {todo!();};
        if *s < min {
            return *s;
        }
        return min;
    });

}