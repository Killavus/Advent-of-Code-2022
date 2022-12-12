use anyhow::{anyhow, Error, Result};
use std::{
    collections::HashMap,
    io::{stdin, BufReader, Read},
    str::FromStr,
};

#[derive(Debug)]
struct HeightMap {
    start: (usize, usize),
    end: (usize, usize),
    heights: HashMap<(usize, usize), u64>,
}

impl FromStr for HeightMap {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut heights = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, height) in line.bytes().enumerate() {
                match height {
                    b'S' => {
                        start = Some((x, y));
                        heights.insert((x, y), 0);
                    }
                    b'E' => {
                        end = Some((x, y));
                        heights.insert((x, y), 25);
                    }
                    byte => {
                        heights.insert((x, y), (byte - b'a') as u64);
                    }
                }
            }
        }

        let start = start.ok_or_else(|| anyhow!("failed to find a start point"))?;
        let end = end.ok_or_else(|| anyhow!("failed to find an end point"))?;

        Ok(Self {
            start,
            end,
            heights,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct PathSegment(usize, usize, u64);

impl PartialEq for PathSegment {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl PartialOrd for PathSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Reverse;

        Reverse(self.2).partial_cmp(&Reverse(other.2))
    }
}

impl Eq for PathSegment {}
impl Ord for PathSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Reverse;

        Reverse(self.2).cmp(&Reverse(other.2))
    }
}

impl HeightMap {
    fn neighbours((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        [
            (x as isize + 1, y as isize),
            (x as isize - 1, y as isize),
            (x as isize, y as isize + 1),
            (x as isize, y as isize - 1),
        ]
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
    }

    fn shortest_path(&self, start: (usize, usize)) -> Option<u64> {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let mut heap = BinaryHeap::from_iter([PathSegment(start.0, start.1, 0)].iter().copied());
        let mut used: HashSet<(usize, usize)> = HashSet::new();
        used.insert((self.start.0, self.start.1));

        while !heap.is_empty() {
            let PathSegment(x, y, cost) = heap.pop().unwrap();

            if x == self.end.0 && y == self.end.1 {
                return Some(cost);
            }

            let height = self.heights.get(&(x, y)).copied().unwrap();

            for (nx, ny) in Self::neighbours((x, y)) {
                if !used.contains(&(nx, ny)) {
                    if let Some(nheight) = self.heights.get(&(nx, ny)).copied() {
                        if height + 1 >= nheight {
                            used.insert((nx, ny));
                            heap.push(PathSegment(nx, ny, cost + 1));
                        }
                    }
                }
            }
        }

        None
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    BufReader::new(stdin()).read_to_string(&mut input)?;
    let heightmap: HeightMap = input.parse()?;

    let cost = heightmap.shortest_path(heightmap.start);
    if let Some(cost) = cost {
        println!("Found path from S to E with cost {}", cost);
    } else {
        println!("Couldn't find the path to destination E from starting point.");
    }

    let min_starting_elevation_cost = heightmap
        .heights
        .iter()
        .filter(|(_, v)| **v == 0)
        .flat_map(|(k, _)| heightmap.shortest_path(*k))
        .min();

    if let Some(min_starting_elevation_cost) = min_starting_elevation_cost {
        println!(
            "The shortest path from lowest points is {}",
            min_starting_elevation_cost
        )
    } else {
        println!("Couldn't find path from any of the lowest points.");
    }

    Ok(())
}
