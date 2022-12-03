use std::collections::HashSet;
use std::io::{prelude::*, stdin, BufReader};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn error_priorities(reader: impl BufRead) -> AppResult<usize> {
    let mut total = 0;

    for rucksack in reader.split(0x0A) {
        let rucksack = rucksack?;
        let half_index = rucksack.len() / 2;

        let (first_compartment, second_compartment) =
            (&rucksack[0..half_index], &rucksack[half_index..]);

        let misplaced_item = first_compartment
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&second_compartment.iter().copied().collect())
            .into_iter()
            .copied()
            .next()
            .expect("there is at least one mispaced item in compartments");

        let item_score = if (b'a'..=b'z').contains(&misplaced_item) {
            misplaced_item - b'a' + 1
        } else {
            misplaced_item - b'A' + 27
        } as usize;

        total += item_score;
    }

    Ok(total)
}

fn main() -> AppResult<()> {
    println!(
        "Total priority of misplaced items is {}",
        error_priorities(BufReader::new(stdin()))?
    );

    Ok(())
}
