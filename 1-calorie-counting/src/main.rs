use std::io::stdin;
use std::io::BufReader;

use std::io::prelude::*;

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

struct ElfCarryIter {
    reader: Box<dyn BufRead>,
    line: String,
}

impl ElfCarryIter {
    fn new(reader: impl BufRead + 'static) -> Self {
        Self {
            reader: Box::new(reader),
            line: String::with_capacity(12),
        }
    }
}

impl Iterator for ElfCarryIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut elf_calories = 0;

        loop {
            let bytes_read = self.reader.read_line(&mut self.line);
            match bytes_read {
                Ok(0) => break None,
                Ok(_) => {
                    let content = self.line.trim();
                    if content.is_empty() {
                        break Some(elf_calories);
                    } else {
                        let calories: usize = content.parse().expect("input should be valid");
                        elf_calories += calories;
                    }
                    self.line.truncate(0);
                }
                Err(_) => break None,
            }
        }
    }
}

fn most_calories_k_elves_carry(reader: impl BufRead + 'static, k: usize) -> AppResult<usize> {
    let iter = ElfCarryIter::new(reader);
    let mut result = Vec::with_capacity(k);
    let mut current_min_idx = 0;

    for elf_calories in iter {
        if result.len() < k {
            result.push(elf_calories);
            current_min_idx = result
                .iter()
                .enumerate()
                .min_by_key(|(_, value)| *value)
                .unwrap()
                .0;

            continue;
        }

        if elf_calories > result[current_min_idx] {
            result[current_min_idx] = elf_calories;
            current_min_idx = result
                .iter()
                .enumerate()
                .min_by_key(|(_, value)| *value)
                .unwrap()
                .0;
        }
    }

    Ok(result.into_iter().sum())
}

fn main() {
    // println!(
    //     "Elf carry at most {} calories",
    //     most_calories_k_elves_carry(BufReader::new(stdin()), 1).expect("input should be valid")
    // );
    println!(
        "Three elves carry at most {} calories",
        most_calories_k_elves_carry(BufReader::new(stdin()), 3).expect("input should be valid")
    )
}

// Quick & dirty solution with log(n) added in & whole file in memory:
// fn main() {
//     let mut input = String::new();
//     stdin().read_to_string(&mut input).ok();
//     let mut elves = input.lines().fold(vec![0], |mut acc, l| {
//         l.trim().parse::<usize>()
//             .map(|v| *acc.last_mut().unwrap() += v)
//             .map_err(|_| acc.push(0))
//             .ok();
//         acc
//     });
//     elves.sort_unstable();
//     println!("{} {}", elves.last().unwrap(), elves.iter().rev().take(3).sum::<usize>());
// }
