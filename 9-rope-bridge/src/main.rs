use anyhow::Result;
use std::{
    cmp::Ordering,
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Move(Direction, usize);

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use anyhow::anyhow;

        Ok(match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => Err(anyhow!("failed to parse direction: {}", s))?,
        })
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s[0..1].parse()?, s[2..].parse()?))
    }
}

fn read(reader: impl BufRead) -> impl Iterator<Item = Result<Move>> {
    use anyhow::anyhow;

    reader.lines().map(|line| {
        line.map_err(Into::into)
            .and_then(|line| line.parse())
            .map_err(|e| anyhow!("failed to parse line: {}", e))
    })
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct Knot {
    head: Point,
    tail: Point,
}

impl Knot {
    fn new() -> Self {
        Self {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        }
    }

    fn step(&mut self, direction: &Direction) {
        use Direction::*;

        match direction {
            Left => {
                self.head.x -= 1;
            }
            Right => {
                self.head.x += 1;
            }
            Up => {
                self.head.y -= 1;
            }
            Down => {
                self.head.y += 1;
            }
        }

        self.adjust_tail()
    }

    fn adjust_tail(&mut self) {
        let needs_adjustment = ![1, 0, -1]
            .into_iter()
            .flat_map(|x| [1, 0, -1].into_iter().map(move |y| (x, y)))
            .any(|(dx, dy)| self.head.x + dx == self.tail.x && self.head.y + dy == self.tail.y);

        if needs_adjustment {
            let (adjx, adjy) = self.tail_adjustment();

            self.tail.x += adjx;
            self.tail.y += adjy;
        }
    }

    fn tail_adjustment(&self) -> (isize, isize) {
        let (tx, ty) = (self.tail.x, self.tail.y);
        let (hx, hy) = (self.head.x, self.head.y);

        (
            match tx.cmp(&hx) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                _ => 1,
            },
            match ty.cmp(&hy) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                _ => 1,
            },
        )
    }
}

fn main() -> Result<()> {
    let mut set = HashSet::from([Point { x: 0, y: 0 }]);

    let mut knot = Knot::new();
    for move_cmd in read(BufReader::new(stdin())) {
        let Move(direction, step) = move_cmd?;

        (0..step).for_each(|_| {
            knot.step(&direction);
            set.insert(knot.tail);
        })
    }

    println!("Knot tail was in {} unique positions", set.len());

    Ok(())
}
