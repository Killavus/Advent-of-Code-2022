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

fn most_calories_elf_carry(reader: impl BufRead + 'static) -> AppResult<usize> {
    Ok(ElfCarryIter::new(reader).max().expect("input is valid"))
}

fn main() {
    let most_calories_elf_carry =
        most_calories_elf_carry(BufReader::new(stdin())).expect("input should be valid");

    println!("Elf carry at most {} calories", most_calories_elf_carry);
}
