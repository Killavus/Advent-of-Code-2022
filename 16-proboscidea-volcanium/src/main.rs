use anyhow::{anyhow, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Hash)]
struct CaveNode(String, i64, Vec<String>);

impl FromStr for CaveNode {
    type Err = Error;

    fn from_str(node: &str) -> Result<Self, Self::Err> {
        let mut valve_tunnels = node.split("; ");

        let valve = valve_tunnels
            .next()
            .ok_or_else(|| anyhow!("failed to get valve part"))?;

        let valve_id = valve
            .split(" has ")
            .next()
            .map(|x| x[6..].to_owned())
            .ok_or_else(|| anyhow!("failed to get valve id"))?;

        let flow_rate = valve
            .split("flow rate=")
            .last()
            .map(|rate| rate.parse())
            .ok_or_else(|| anyhow!("failed to get flow rate"))??;

        let tunnels = valve_tunnels
            .next()
            .ok_or_else(|| anyhow!("failed to get tunnels part"))?;

        let neighbours = if tunnels.contains("tunnels") {
            tunnels
                .split("valves ")
                .last()
                .map(|lines| {
                    lines
                        .split(", ")
                        .map(|line| line.to_owned())
                        .collect::<Vec<_>>()
                })
                .ok_or_else(|| anyhow!("failed to read tunnels"))?
        } else {
            tunnels
                .split("valve ")
                .last()
                .map(|part| vec![part.to_owned()])
                .ok_or_else(|| anyhow!("failed to read tunnel"))?
        };

        Ok(Self(valve_id, flow_rate, neighbours))
    }
}

#[derive(Debug)]
struct CaveSystem(HashMap<String, CaveNode>);

fn read(reader: impl BufRead) -> Result<CaveSystem> {
    let nodes = reader
        .lines()
        .map(|line| line.map_err(Into::into).and_then(|line| line.parse()))
        .collect::<Result<Vec<CaveNode>>>()?;

    Ok(CaveSystem(
        nodes
            .into_iter()
            .map(|node| (node.0.clone(), node))
            .collect(),
    ))
}

fn floyd_warshall(cave_system: &CaveSystem) -> HashMap<(String, String), i64> {
    let mut result = HashMap::new();
    cave_system.0.keys().for_each(|key| {
        result.insert((key.clone(), key.clone()), 0);
    });

    for (v, node) in cave_system.0.iter() {
        for v2 in node.2.iter() {
            result.insert((v.to_owned(), v2.to_owned()), 1);
            result.insert((v2.to_owned(), v.to_owned()), 1);
        }
    }

    for u in cave_system.0.keys() {
        for v1 in cave_system.0.keys() {
            for v2 in cave_system.0.keys() {
                let v1_v2 = (v1.clone(), v2.clone());
                let v1_u = (v1.clone(), u.clone());
                let u_v2 = (u.clone(), v2.clone());

                if result.get(&v1_v2).copied().unwrap_or(i64::MAX)
                    > result
                        .get(&v1_u)
                        .copied()
                        .unwrap_or(i64::MAX)
                        .saturating_add(result.get(&u_v2).copied().unwrap_or(i64::MAX))
                {
                    result.insert(
                        v1_v2,
                        result
                            .get(&v1_u)
                            .copied()
                            .unwrap_or(i64::MAX)
                            .saturating_add(result.get(&u_v2).copied().unwrap_or(i64::MAX)),
                    );
                }
            }
        }
    }

    for key in cave_system.0.keys() {
        result.remove(&(key.to_owned(), key.to_owned()));
    }

    result
}

fn all_relief_paths(
    start: &String,
    valves: &[String],
    system: &CaveSystem,
    path_used: i64,
    path_so_far: String,
    pressure_so_far: i64,
    distances: &HashMap<(String, String), i64>,
    mut used: HashSet<String>,
    paths: &mut HashMap<String, i64>,
    total_time: i64,
) {
    used.insert(start.to_string());
    let relief_pressure =
        (total_time - (path_used + 1)) * system.0.get(start).map(|n| n.1).unwrap();
    paths.insert(path_so_far.clone(), pressure_so_far + relief_pressure);

    for valve in valves {
        if !used.contains(valve) {
            let distance = distances
                .get(&(start.to_string(), valve.to_string()))
                .copied()
                .unwrap();

            if distance + path_used < total_time {
                all_relief_paths(
                    valve,
                    valves,
                    system,
                    distance + path_used + 1,
                    {
                        let mut path = path_so_far.clone();
                        path.push(':');
                        path.push_str(valve.as_str());
                        path
                    },
                    pressure_so_far + relief_pressure,
                    distances,
                    used.clone(),
                    paths,
                    total_time,
                );
            }
        }
    }
}

fn main() -> Result<()> {
    let cave_system = read(BufReader::new(stdin()))?;
    let distances = floyd_warshall(&cave_system);
    let meaningful_valves = cave_system
        .0
        .values()
        .filter(|node| node.1 > 0)
        .map(|node| node.0.clone())
        .collect::<Vec<_>>();

    let mut solo_paths = HashMap::new();
    meaningful_valves.iter().for_each(|valve| {
        all_relief_paths(
            valve,
            &meaningful_valves,
            &cave_system,
            distances
                .get(&("AA".to_string(), valve.clone()))
                .copied()
                .unwrap(),
            valve.to_string(),
            0,
            &distances,
            HashSet::new(),
            &mut solo_paths,
            30,
        )
    });

    let max_pressure_solo = solo_paths.values().copied().max().unwrap();

    println!(
        "Maximum pressure you can relieve by yourself is {}",
        max_pressure_solo
    );

    let mut with_elephant_paths = HashMap::new();
    meaningful_valves.iter().for_each(|valve| {
        all_relief_paths(
            valve,
            &meaningful_valves,
            &cave_system,
            distances
                .get(&("AA".to_string(), valve.clone()))
                .copied()
                .unwrap(),
            valve.to_string(),
            0,
            &distances,
            HashSet::new(),
            &mut with_elephant_paths,
            26,
        )
    });

    let all_paths = with_elephant_paths
        .into_iter()
        .map(|(key, value)| {
            (
                key.split(':').map(|f| f.to_owned()).collect::<HashSet<_>>(),
                value,
            )
        })
        .collect::<Vec<_>>();

    let mut max_pressure_elephant = 0;
    for (path, cost) in all_paths.iter() {
        for (path2, cost2) in all_paths.iter() {
            if path.is_disjoint(path2) {
                max_pressure_elephant = max_pressure_elephant.max(cost + cost2);
            }
        }
    }

    println!(
        "Max pressure you can relieve with elephant is {}",
        max_pressure_elephant
    );

    Ok(())
}
