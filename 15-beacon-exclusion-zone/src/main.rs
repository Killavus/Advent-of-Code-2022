use derive_more::Display;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "{{{}, {}}}", _0, _1)]
struct Position(i64, i64);

#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "sensor: {} | beacon: {}", sensor, closest_beacon)]
struct SensorReport {
    sensor: Position,
    closest_beacon: Position,
}

impl FromStr for SensorReport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reading = s.split(": ");
        let sensor = reading
            .next()
            .ok_or_else(|| anyhow!("could not find sensor position reading"))?[10..]
            .parse()?;
        let closest_beacon = reading
            .next()
            .ok_or_else(|| anyhow!("could not find beacon position reading"))?[21..]
            .parse()?;

        Ok(Self {
            sensor,
            closest_beacon,
        })
    }
}

impl Position {
    fn distance(&self, other: &Self) -> i64 {
        self.x_distance(other) + self.y_distance(other)
    }

    fn y_distance(&self, other: &Self) -> i64 {
        (other.1 - self.1).abs()
    }

    fn x_distance(&self, other: &Self) -> i64 {
        (other.0 - self.0).abs()
    }
}

use std::{
    io::{stdin, BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xy = s.split(", ").map(|coord| coord[2..].parse::<i64>());
        let x = xy
            .next()
            .ok_or_else(|| anyhow!("could not find x coord"))??;
        let y = xy
            .next()
            .ok_or_else(|| anyhow!("could not find y coord"))??;

        Ok(Self(x, y))
    }
}

fn read(reader: impl BufRead) -> Result<Vec<SensorReport>> {
    reader
        .lines()
        .map(|line| line.map_err(Into::into).and_then(|line| line.parse()))
        .collect::<Result<Vec<_>>>()
}

#[derive(Debug, Clone)]
struct RangeSet {
    ranges: Vec<(i64, i64)>,
}

impl RangeSet {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn append(&mut self, range: RangeInclusive<i64>) {
        // drain_filter would be way more effective here...
        let mut idx = 0;
        let mut overlaps = vec![];
        while idx < self.ranges.len() {
            let (start, end) = self.ranges[idx];
            let overlapping = (start..=end).contains(range.start())
                || (start..=end).contains(range.end())
                || range.contains(&start)
                || range.contains(&end);

            if overlapping {
                overlaps.push(self.ranges.remove(idx));
            } else {
                idx += 1;
            }
        }

        let target_range = overlaps
            .into_iter()
            .fold(range, |result_range, (start, end)| {
                *result_range.start().min(&start)..=*result_range.end().max(&end)
            });

        self.ranges
            .push((*target_range.start(), *target_range.end()))
    }

    fn covers(&self, point: i64) -> bool {
        self.ranges
            .iter()
            .any(|(start, end)| (*start..=*end).contains(&point))
    }

    fn coverage(&self) -> usize {
        self.ranges
            .iter()
            .map(|(start, end)| (end + 1 - start).unsigned_abs() as usize)
            .sum()
    }

    fn clamp_to_and_sort(&mut self, min_x: i64, max_x: i64) {
        let clamp_range = min_x..=max_x;
        let mut to_delete = vec![];

        for idx in 0..self.ranges.len() {
            let (start, end) = self.ranges[idx];

            if clamp_range.contains(&start)
                || clamp_range.contains(&end)
                || (start..=end).contains(&min_x)
                || (start..=end).contains(&max_x)
            {
                self.ranges[idx] = (start.max(min_x), end.min(max_x));
            } else {
                to_delete.push(idx);
            }
        }

        for idx in to_delete {
            self.ranges.remove(idx);
        }

        self.ranges.sort_by_key(|(start, _)| *start);
    }

    fn first_gap(&self) -> Option<i64> {
        for ranges in self.ranges.as_slice().windows(2) {
            let (_, end) = ranges[0];
            let (start, _) = ranges[1];

            if start - end == 2 {
                return Some(end + 1);
            }
        }

        None
    }
}

fn objects_in_row(reports: &[SensorReport], target_y: i64) -> HashSet<i64> {
    let mut existing_in_row = HashSet::new();
    for report in reports {
        [report.sensor, report.closest_beacon]
            .iter()
            .filter(|pos| pos.1 == target_y)
            .for_each(|pos| {
                existing_in_row.insert(pos.0);
            })
    }

    existing_in_row
}

fn row_coverage_ranges(reports: &[SensorReport], target_y: i64) -> RangeSet {
    let mut range_set = RangeSet::new();

    for report in reports {
        let exclusion_distance = report.sensor.distance(&report.closest_beacon);
        let below_sensor_at_target_y = Position(report.sensor.0, target_y);
        let target_distance = report.sensor.distance(&below_sensor_at_target_y);

        if target_distance <= exclusion_distance {
            let x_distance = exclusion_distance - target_distance;
            let area_slice = report.sensor.0 - x_distance..=report.sensor.0 + x_distance;

            range_set.append(area_slice);
        }
    }

    range_set
}

fn no_beacon_positions_at_y(reports: &[SensorReport], target_y: i64) -> usize {
    let range_set = row_coverage_ranges(reports, target_y);
    let in_row = objects_in_row(reports, target_y);

    range_set.coverage() - in_row.into_iter().filter(|x| range_set.covers(*x)).count()
}

fn search_for_distress(limit: i64, reports: &[SensorReport]) -> Option<Position> {
    for y in 0..=limit {
        let mut range_set = row_coverage_ranges(reports, y);
        range_set.clamp_to_and_sort(0, limit);

        if range_set.coverage() == limit as usize {
            return range_set.first_gap().map(|x| Position(x, y));
        }
    }

    None
}

fn main() -> Result<()> {
    let sensor_reports = read(BufReader::new(stdin()))?;

    println!(
        "At row 10, there are {} positions where beacon cannot be present.",
        no_beacon_positions_at_y(&sensor_reports, 10)
    );
    println!(
        "At row 2000000, there are {} positions where beacon cannot be present.",
        no_beacon_positions_at_y(&sensor_reports, 2000000)
    );

    if let Some(distress_beacon_position) = search_for_distress(20, &sensor_reports) {
        println!(
            "Found distress signal while searching in (0, 20) square: {}",
            distress_beacon_position
        );

        let tuning_frequency = distress_beacon_position.0 * 4_000_000 + distress_beacon_position.1;
        println!("It's tuning frequency is {}", tuning_frequency);
    } else if let Some(distress_beacon_position) = search_for_distress(4_000_000, &sensor_reports) {
        println!(
            "Found distress signal while searching in (0, 4000000) square: {}",
            distress_beacon_position
        );

        let tuning_frequency = distress_beacon_position.0 * 4_000_000 + distress_beacon_position.1;
        println!("It's tuning frequency is {}", tuning_frequency);
    }

    Ok(())
}
