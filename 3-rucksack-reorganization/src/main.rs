use std::collections::HashSet;
use std::io::{prelude::*, stdin, BufReader};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn reoccuring_item_type(item_type_collections: &[&[u8]]) -> u8 {
    let intersection_set = item_type_collections
        .iter()
        .copied()
        .map(|collection| collection.iter().copied().collect::<HashSet<_>>())
        .reduce(|result, next| result.intersection(&next).copied().collect());

    intersection_set
        .expect("at least one collection is provided")
        .into_iter()
        .next()
        .expect("there is at least one reocurring item type")
}

fn item_type_score(item_type: u8) -> usize {
    (if (b'a'..=b'z').contains(&item_type) {
        item_type - b'a' + 1
    } else {
        item_type - b'A' + 27
    }) as usize
}

fn reorganization_priorities(reader: impl BufRead) -> AppResult<(usize, usize)> {
    let mut total_compartments = 0;
    let mut total_group = 0;

    let mut rucksacks = reader.split(b'\n');

    loop {
        let (rucksack_a, rucksack_b, rucksack_c) =
            (rucksacks.next(), rucksacks.next(), rucksacks.next());

        if rucksack_a.is_none() || rucksack_b.is_none() || rucksack_c.is_none() {
            break;
        }

        // SAFETY: Checked for None in previous conditional.
        let (mut rucksack_a, mut rucksack_b, mut rucksack_c) = (
            rucksack_a.unwrap()?,
            rucksack_b.unwrap()?,
            rucksack_c.unwrap()?,
        );

        // Correctness: If input contains carriage return (\r), it needs to be removed.
        if let Some(b) = rucksack_a.last().copied() {
            if b == b'\r' {
                rucksack_a.pop();
            }
        }

        if let Some(b) = rucksack_b.last().copied() {
            if b == b'\r' {
                rucksack_b.pop();
            }
        }

        if let Some(b) = rucksack_c.last().copied() {
            if b == b'\r' {
                rucksack_c.pop();
            }
        }

        for item in &[&rucksack_a, &rucksack_b, &rucksack_c] {
            let half_idx = item.len() / 2;
            let (first_compartment, second_compartment) = (&item[0..half_idx], &item[half_idx..]);
            let misplaced_item = reoccuring_item_type(&[first_compartment, second_compartment]);
            total_compartments += item_type_score(misplaced_item);
        }

        let group_badge = reoccuring_item_type(&[&rucksack_a, &rucksack_b, &rucksack_c]);
        total_group += item_type_score(group_badge);
    }

    Ok((total_compartments, total_group))
}

fn main() -> AppResult<()> {
    let (total_compartments, total_group) = reorganization_priorities(BufReader::new(stdin()))?;

    println!(
        "Total priority of misplaced items in compartments is {}",
        total_compartments
    );

    println!("Total priority of group badges is {}", total_group);

    Ok(())
}
