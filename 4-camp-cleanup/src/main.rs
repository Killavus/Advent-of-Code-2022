use std::{
    io::{prelude::*, stdin, BufReader},
    str::FromStr,
};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Pair(u64, u64);

#[derive(Debug)]
struct Assignment {
    first: Pair,
    second: Pair,
}

impl FromStr for Pair {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s.split('-').collect::<Vec<_>>();
        Ok(Self(pair[0].parse()?, pair[1].parse()?))
    }
}

impl Pair {
    fn fully_contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

impl FromStr for Assignment {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s.split(',').collect::<Vec<_>>();

        Ok(Self {
            first: pair[0].parse()?,
            second: pair[1].parse()?,
        })
    }
}

fn overlapping_pairs(reader: impl BufRead) -> AppResult<usize> {
    let mut overlapping = 0;

    for line in reader.lines() {
        let assignment = line?.parse::<Assignment>()?;

        overlapping += usize::from(
            assignment.first.fully_contains(&assignment.second)
                || assignment.second.fully_contains(&assignment.first),
        );
    }

    Ok(overlapping)
}

fn main() -> AppResult<()> {
    let overlaps = overlapping_pairs(BufReader::new(stdin()))?;
    println!(
        "There are {} full overlaps between elf assignments",
        overlaps
    );

    Ok(())
}
