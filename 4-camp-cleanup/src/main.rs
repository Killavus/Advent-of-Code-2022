use std::{
    io::{prelude::*, stdin, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Pair(RangeInclusive<u64>);

#[derive(Debug)]
struct Assignment {
    first: Pair,
    second: Pair,
}

impl FromStr for Pair {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s.split('-').collect::<Vec<_>>();
        Ok(Self(pair[0].parse()?..=pair[1].parse()?))
    }
}

impl Pair {
    fn fully_contains(&self, other: &Self) -> bool {
        (self.0.contains(other.0.start()) && self.0.contains(other.0.end()))
            || (other.0.contains(self.0.start()) && other.0.contains(self.0.end()))
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.0.contains(other.0.start())
            || self.0.contains(other.0.end())
            || other.0.contains(self.0.start())
            || other.0.contains(self.0.end())
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

fn overlapping_assignments(reader: impl BufRead) -> AppResult<(usize, usize)> {
    let mut fully_overlapping = 0;
    let mut overlapping = 0;

    for line in reader.lines() {
        let assignment = line?.parse::<Assignment>()?;

        fully_overlapping += usize::from(assignment.first.fully_contains(&assignment.second));
        overlapping += usize::from(assignment.first.overlaps(&assignment.second));
    }

    Ok((fully_overlapping, overlapping))
}

fn main() -> AppResult<()> {
    let (full_overlaps, overlaps) = overlapping_assignments(BufReader::new(stdin()))?;
    println!(
        "There are {} full overlaps between elf assignments",
        full_overlaps
    );

    println!("There are {} overlaps between elf assignments", overlaps);

    Ok(())
}
