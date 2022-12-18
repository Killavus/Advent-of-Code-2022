use anyhow::{Error, Result};
use std::{
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UnitCube {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for UnitCube {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xyz = s
            .split(',')
            .map(|coord| coord.parse().map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        })
    }
}

fn read(reader: impl BufRead) -> Result<Vec<UnitCube>> {
    reader
        .lines()
        .map(|line| line.map_err(Into::into).and_then(|line| line.parse()))
        .collect::<Result<_>>()
}

impl UnitCube {
    fn adjacent(&self, other: &Self) -> bool {
        self.adjacent_in_x(other) || self.adjacent_in_y(other) || self.adjacent_in_z(other)
    }

    fn adjacent_in_x(&self, other: &Self) -> bool {
        [-1, 1].into_iter().any(|d| {
            &UnitCube {
                x: self.x + d,
                y: self.y,
                z: self.z,
            } == other
        })
    }

    fn adjacent_in_y(&self, other: &Self) -> bool {
        [-1, 1].into_iter().any(|d| {
            &UnitCube {
                x: self.x,
                y: self.y + d,
                z: self.z,
            } == other
        })
    }
    fn adjacent_in_z(&self, other: &Self) -> bool {
        [-1, 1].into_iter().any(|d| {
            &UnitCube {
                x: self.x,
                y: self.y,
                z: self.z + d,
            } == other
        })
    }
}

fn main() -> Result<()> {
    let cubes = read(BufReader::new(stdin()))?;

    let mut total = 0;
    for cube in cubes.iter() {
        let mut sides = 6;
        for other_cube in cubes.iter() {
            if cube == other_cube {
                continue;
            }

            if cube.adjacent(other_cube) {
                sides -= 1;
            }
        }

        total += sides;
    }

    println!("Total surface area of the droplet is {}", total);

    Ok(())
}
