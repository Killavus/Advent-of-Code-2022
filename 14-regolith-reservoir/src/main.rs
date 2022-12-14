use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

use anyhow::{Error, Result};

#[derive(Debug, Clone, Copy)]
struct LinePoint(i64, i64);

impl FromStr for LinePoint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xy = s
            .split(',')
            .map(|n| n.parse().map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(xy[0], xy[1]))
    }
}

#[derive(Debug)]
struct PolyLine(Vec<LinePoint>);

impl FromStr for PolyLine {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            line.split(" -> ")
                .map(str::parse)
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

fn read(reader: impl BufRead) -> Result<Vec<PolyLine>> {
    reader
        .lines()
        .map(|line| line.map_err(Into::into).and_then(|line| line.parse()))
        .collect::<Result<Vec<_>>>()
}

fn right_range(start: i64, end: i64) -> RangeInclusive<i64> {
    start.min(end)..=start.max(end)
}

impl PolyLine {
    fn collides_with(&self, point: &(i64, i64)) -> bool {
        self.0.as_slice().windows(2).any(|window| {
            let start = window[0];
            let end = window[1];
            right_range(start.0, end.0).contains(&point.0)
                && right_range(start.1, end.1).contains(&point.1)
        })
    }

    fn bounding_box(&self) -> (i64, i64, i64, i64) {
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (i64::MAX, i64::MIN, 0, i64::MIN);

        for subline in self.0.as_slice().windows(2) {
            let x_range = right_range(subline[0].0, subline[1].0);
            let y_range = right_range(subline[0].1, subline[1].1);

            if min_x > *x_range.start() {
                min_x = *x_range.start();
            }

            if max_x < *x_range.end() {
                max_x = *x_range.end();
            }

            if min_y > *y_range.start() {
                min_y = *y_range.start();
            }

            if max_y < *y_range.end() {
                max_y = *y_range.end();
            }
        }

        (min_x, max_x, min_y, max_y)
    }
}

fn total_bounding_box(structures: &[PolyLine]) -> (i64, i64, i64, i64) {
    structures.iter().fold(
        (i64::MAX, i64::MIN, 0, i64::MIN),
        |(min_x, max_x, min_y, max_y), polyline| {
            let bb = polyline.bounding_box();

            (
                min_x.min(bb.0),
                max_x.max(bb.1),
                min_y.min(bb.2),
                max_y.max(bb.3),
            )
        },
    )
}

fn simulate_sand(
    starting_position: (i64, i64),
    existing_sand: &HashSet<(i64, i64)>,
    &(min_x, max_x, min_y, max_y): &(i64, i64, i64, i64),
    rocks: &[PolyLine],
) -> Option<(i64, i64)> {
    let (mut x, mut y) = starting_position;
    let possible_moves = &[(0, 1), (-1, 1), (1, 1)];

    'main: loop {
        if !(min_y..=max_y).contains(&y) || !(min_x..=max_x).contains(&x) {
            return None;
        }

        for (dx, dy) in possible_moves {
            let new_position = (x + dx, y + dy);

            if !existing_sand.contains(&new_position)
                && !rocks.iter().any(|rock| rock.collides_with(&new_position))
            {
                x = new_position.0;
                y = new_position.1;
                continue 'main;
            }
        }

        return Some((x, y));
    }
}

fn main() -> Result<()> {
    let mut rock_structures = read(BufReader::new(stdin()))?;
    let mut bounding_box = total_bounding_box(&rock_structures);
    let mut sand_grains: HashSet<(i64, i64)> = HashSet::new();

    while let Some((sx, sy)) =
        simulate_sand((500, 0), &sand_grains, &bounding_box, &rock_structures)
    {
        sand_grains.insert((sx, sy));
    }

    println!(
        "{} grains of sand come to rest before falling into the abyss",
        sand_grains.len()
    );

    // Add floor:
    rock_structures.push(PolyLine(vec![
        LinePoint(i64::MIN, bounding_box.3 + 2),
        LinePoint(i64::MAX, bounding_box.3 + 2),
    ]));
    bounding_box.0 = i64::MIN;
    bounding_box.1 = i64::MAX;
    bounding_box.3 += 2;

    while let Some((sx, sy)) =
        simulate_sand((500, 0), &sand_grains, &bounding_box, &rock_structures)
    {
        sand_grains.insert((sx, sy));

        if sx == 500 && sy == 0 {
            break;
        }
    }

    println!(
        "{} grains of sand falls into the cave until it blocks the source entirely",
        sand_grains.len()
    );

    Ok(())
}
