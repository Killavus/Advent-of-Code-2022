use std::io::stdin;
use std::io::BufReader;

use std::io::prelude::*;

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn most_calories_elf_carry(mut reader: impl BufRead) -> AppResult<usize> {
    let mut line = String::with_capacity(12);
    let mut current_elf_calories = 0;
    let mut max_elf_calories = 0;

    loop {
        let bytes_read = reader.read_line(&mut line);

        if let Ok(bytes_read) = bytes_read {
            if bytes_read == 0 {
                break;
            }
        } else {
            break;
        }

        let content = line.trim();
        if content.is_empty() {
            max_elf_calories = max_elf_calories.max(current_elf_calories);
            current_elf_calories = 0;
        } else {
            let calories: usize = content.parse().expect("input should be valid");
            current_elf_calories += calories;
        }

        line.truncate(0);
    }

    Ok(max_elf_calories)
}

fn main() {
    let most_calories_elf_carry =
        most_calories_elf_carry(BufReader::new(stdin())).expect("input should be valid");

    println!("Elf carry at most {} calories", most_calories_elf_carry);
}
