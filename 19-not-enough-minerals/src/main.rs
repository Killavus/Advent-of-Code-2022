use anyhow::{anyhow, Error, Result};
use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore_robot_cost: u64,
    clay_robot_cost: u64,
    obsidian_robot_cost: (u64, u64),
    geode_robot_cost: (u64, u64),
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut header_content = line.split(": ");
        let id = header_content
            .next()
            .ok_or_else(|| anyhow!("failed to find header in input: {}", line))?[10..]
            .parse()?;
        let mut ore_costs = header_content
            .next()
            .ok_or_else(|| anyhow!("failed to find content in input: {}", line))?
            .split(". ");
        let ore_robot_cost = ore_costs
            .next()
            .ok_or_else(|| anyhow!("failed to find ore robot cost in input: {}", line))?
            .split(' ')
            .flat_map(|word| word.parse::<u64>())
            .collect::<Vec<_>>();
        let clay_robot_cost = ore_costs
            .next()
            .ok_or_else(|| anyhow!("failed to find clay robot cost in input: {}", line))?
            .split(' ')
            .flat_map(|word| word.parse::<u64>())
            .collect::<Vec<_>>();
        let obsidian_robot_cost = ore_costs
            .next()
            .ok_or_else(|| anyhow!("failed to find obsidian robot cost in input: {}", line))?
            .split(' ')
            .flat_map(|word| word.parse::<u64>())
            .collect::<Vec<_>>();
        let geode_robot_cost = ore_costs
            .next()
            .ok_or_else(|| anyhow!("failed to find geode robot cost in input: {}", line))?
            .split(' ')
            .flat_map(|word| word.parse::<u64>())
            .collect::<Vec<_>>();

        for (collection, expected_len) in vec![
            &ore_robot_cost,
            &clay_robot_cost,
            &obsidian_robot_cost,
            &geode_robot_cost,
        ]
        .into_iter()
        .zip(vec![1, 1, 2, 2])
        {
            if collection.len() != expected_len {
                return Err(anyhow!(
                    "wrong number of costs for robot - expected {}, got {}",
                    expected_len,
                    collection.len()
                ));
            }
        }

        Ok(Self {
            id,
            ore_robot_cost: ore_robot_cost[0],
            clay_robot_cost: clay_robot_cost[0],
            obsidian_robot_cost: (obsidian_robot_cost[0], obsidian_robot_cost[1]),
            geode_robot_cost: (geode_robot_cost[0], geode_robot_cost[1]),
        })
    }
}

fn read(reader: impl BufRead) -> Result<Vec<Blueprint>> {
    reader
        .lines()
        .map(|line| line.map_err(Into::into).and_then(|line| line.parse()))
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
    ore_robots: u64,
    clay_robots: u64,
    obsidian_robots: u64,
    geode_robots: u64,
}

impl State {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn buy_ore_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        (self.ore >= blueprint.ore_robot_cost).then_some({
            let node = self.just_harvest();
            Self {
                ore: node.ore - blueprint.ore_robot_cost,
                ore_robots: node.ore_robots + 1,
                ..node
            }
        })
    }

    fn buy_clay_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        (self.ore >= blueprint.clay_robot_cost).then_some({
            let node = self.just_harvest();
            Self {
                ore: node.ore - blueprint.clay_robot_cost,
                clay_robots: node.clay_robots + 1,
                ..node
            }
        })
    }

    fn buy_obsidian_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        (self.ore >= blueprint.obsidian_robot_cost.0
            && self.clay >= blueprint.obsidian_robot_cost.1)
            .then_some({
                let node = self.just_harvest();
                Self {
                    ore: node.ore - blueprint.obsidian_robot_cost.0,
                    clay: node.clay - blueprint.obsidian_robot_cost.1,
                    obsidian_robots: node.obsidian_robots + 1,
                    ..node
                }
            })
    }

    fn buy_geode_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        (self.ore >= blueprint.geode_robot_cost.0 && self.obsidian >= blueprint.geode_robot_cost.1)
            .then_some({
                let node = self.just_harvest();
                Self {
                    ore: node.ore - blueprint.geode_robot_cost.0,
                    obsidian: node.obsidian - blueprint.geode_robot_cost.1,
                    geode_robots: node.geode_robots + 1,
                    ..node
                }
            })
    }

    fn just_harvest(&self) -> Self {
        Self {
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,
            ..*self
        }
    }
}

fn largest_number_of_geodes(
    blueprint: &Blueprint,
    moment: u64,
    node: &State,
    current_max: &mut u64,
    target: u64,
) -> u64 {
    let upper_bound =
        node.geode + ((target - moment) * (2 * node.geode_robots + (target - moment))) / 2;

    if upper_bound < *current_max {
        *current_max
    } else if moment == target {
        *current_max = (*current_max).max(node.geode);
        node.geode
    } else if let Some(node) = node.buy_geode_robot(blueprint) {
        largest_number_of_geodes(blueprint, moment + 1, &node, current_max, target)
    } else {
        let neighbours = [
            (blueprint.geode_robot_cost.1 > node.obsidian_robots)
                .then(|| node.buy_obsidian_robot(blueprint))
                .flatten(),
            (blueprint.obsidian_robot_cost.1 > node.clay_robots)
                .then(|| node.buy_clay_robot(blueprint))
                .flatten(),
            (blueprint.ore_robot_cost > node.ore_robots
                || blueprint.clay_robot_cost > node.ore_robots
                || blueprint.obsidian_robot_cost.0 > node.ore_robots
                || blueprint.geode_robot_cost.0 > node.ore_robots)
                .then(|| node.buy_ore_robot(blueprint))
                .flatten(),
            Some(node.just_harvest()),
        ]
        .into_iter()
        .flatten();

        let mut max_geodes = 0;
        for neighbour in neighbours {
            let max_neighbour =
                largest_number_of_geodes(blueprint, moment + 1, &neighbour, current_max, target);
            max_geodes = max_geodes.max(max_neighbour);
        }

        max_geodes
    }
}

fn main() -> Result<()> {
    let blueprints = read(BufReader::new(stdin()))?;
    /*
    let mut total = 0;

    for blueprint in blueprints.iter() {
        let max_geodes = largest_number_of_geodes(blueprint, 0, &State::new(), &mut 0, 24);
        total += max_geodes * blueprint.id;
    }

    println!("Sum of quality scores is {}", total);
    */

    let mut total = 1;
    for blueprint in blueprints.iter().take(3) {
        let max_geodes = largest_number_of_geodes(blueprint, 0, &State::new(), &mut 0, 32);
        println!("{}", max_geodes);
        total *= max_geodes;
    }

    println!("{}", total);

    Ok(())
}
