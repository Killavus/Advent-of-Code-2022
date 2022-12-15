use derive_more::Display;

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

fn no_beacon_positions_at_y(reports: &[SensorReport], target_y: i64) -> usize {
    use std::collections::HashSet;
    let mut no_beacon_positions = HashSet::new();

    for report in reports {
        let exclusion_distance = report.sensor.distance(&report.closest_beacon);
        let below_sensor_at_target_y = Position(report.sensor.0, target_y);
        let target_distance = report.sensor.distance(&below_sensor_at_target_y);

        if target_distance <= exclusion_distance {
            let x_distance = exclusion_distance - target_distance;
            (report.sensor.0 - x_distance..=report.sensor.0 + x_distance).for_each(|x| {
                no_beacon_positions.insert(x);
            });
        }
    }

    for report in reports {
        [report.sensor, report.closest_beacon]
            .iter()
            .filter(|pos| pos.1 == target_y)
            .for_each(|pos| {
                no_beacon_positions.remove(&pos.0);
            })
    }

    no_beacon_positions.len()
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

    Ok(())
}
