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

type BBox = ((i64, i64), (i64, i64), (i64, i64));
fn count_exterior_area(start: (i64, i64, i64), bbox: &BBox, cubes: &[UnitCube]) -> i64 {
    use std::collections::{HashSet, VecDeque};

    let mut queue = VecDeque::new();
    let mut used = HashSet::new();
    queue.push_front(start);
    used.insert(start);

    let mut result = 0;
    let &((min_x, max_x), (min_y, max_y), (min_z, max_z)) = bbox;

    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        let cube = UnitCube { x, y, z };
        for other in cubes.iter() {
            if other.adjacent(&cube) {
                result += 1;
            }
        }

        for d in [-1, 1] {
            let x_cube = UnitCube { x: x + d, y, z };
            if min_x <= x + d
                && max_x >= x + d
                && !used.contains(&(x + d, y, z))
                && !cubes.contains(&x_cube)
            {
                queue.push_back((x + d, y, z));
                used.insert((x + d, y, z));
            }

            let y_cube = UnitCube { x, y: y + d, z };
            if min_y <= y + d
                && max_y >= y + d
                && !used.contains(&(x, y + d, z))
                && !cubes.contains(&y_cube)
            {
                queue.push_back((x, y + d, z));
                used.insert((x, y + d, z));
            }

            let z_cube = UnitCube { x, y, z: z + d };
            if min_z <= z + d
                && max_z >= z + d
                && !used.contains(&(x, y, z + d))
                && !cubes.contains(&z_cube)
            {
                queue.push_back((x, y, z + d));
                used.insert((x, y, z + d));
            }
        }
    }

    result
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

    println!("Total area of the droplet is {}", total);

    let min_x = cubes.iter().copied().map(|cube| cube.x).min().unwrap();
    let max_x = cubes.iter().copied().map(|cube| cube.x).max().unwrap();
    let min_y = cubes.iter().copied().map(|cube| cube.y).min().unwrap();
    let max_y = cubes.iter().copied().map(|cube| cube.y).max().unwrap();
    let min_z = cubes.iter().copied().map(|cube| cube.z).min().unwrap();
    let max_z = cubes.iter().copied().map(|cube| cube.z).max().unwrap();

    let start = (min_x - 10, min_y - 10, min_z - 10);
    let exterior_area = count_exterior_area(
        start,
        &(
            (min_x - 10, max_x + 10),
            (min_y - 10, max_y + 10),
            (min_z - 10, max_z + 10),
        ),
        &cubes,
    );

    println!("Exterior area of the droplet is {}", exterior_area);

    Ok(())
}
