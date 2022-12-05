use std::{io::stdin, io::BufReader};

mod command;
mod stacks;

use anyhow::Result;

use command::CraneVersion;
use stacks::CrateStacks;

fn main() -> Result<()> {
    let (reader, mut field) = CrateStacks::read(BufReader::new(stdin()))?;

    let mut field_cloned = field.clone();

    for command in command::read_all(reader) {
        let command = command?;

        command.execute(&mut field.stacks, CraneVersion::CraneMover9000);
        command.execute(&mut field_cloned.stacks, CraneVersion::CraneMover9001);
    }

    println!("Crates form a message after crane finishes its work.");
    println!("CraneMover9000: {}", field.message());
    println!("CraneMover9001: {}", field_cloned.message());

    Ok(())
}
