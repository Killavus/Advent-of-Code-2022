use anyhow::{anyhow, Error, Result};
use std::{
    cmp::Reverse,
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Constant(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    test: u64,
    operation: (Operation, Operand),
    throw_if_true: usize,
    throw_if_false: usize,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        lines.next();
        let items = lines.next().ok_or_else(|| anyhow!("wrong input"))?[18..]
            .split(", ")
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        let operation_str = &lines.next().ok_or_else(|| anyhow!("wrong input"))?[23..];
        let operation = match operation_str.as_bytes()[0] {
            b'+' => (
                Operation::Add,
                if &operation_str[2..] == "old" {
                    Operand::Old
                } else {
                    Operand::Constant(operation_str[2..].parse()?)
                },
            ),
            b'*' => (
                Operation::Multiply,
                if &operation_str[2..] == "old" {
                    Operand::Old
                } else {
                    Operand::Constant(operation_str[2..].parse()?)
                },
            ),
            _ => Err(anyhow!("wrong operation"))?,
        };
        let test = lines.next().ok_or_else(|| anyhow!("wrong input"))?[21..].parse()?;
        let throw_if_true = lines.next().ok_or_else(|| anyhow!("wrong input"))?[29..].parse()?;
        let throw_if_false = lines.next().ok_or_else(|| anyhow!("wrong input"))?[30..].parse()?;

        Ok(Self {
            items,
            operation,
            test,
            throw_if_true,
            throw_if_false,
        })
    }
}

fn read(mut reader: impl BufRead) -> Result<Vec<Monkey>> {
    let mut buf = String::with_capacity(160);
    let mut monkeys = vec![];

    let mut lines_read = 0;
    loop {
        let bytes_read = reader.read_line(&mut buf)?;
        if bytes_read == 0 {
            monkeys.push(buf.parse()?);
            break;
        }

        lines_read += 1;
        if lines_read == 7 {
            monkeys.push(buf.parse()?);
            buf.truncate(0);
            lines_read = 0;
        }
    }

    Ok(monkeys)
}

fn process(old: u64, (op, arg): (Operation, Operand)) -> u64 {
    use Operand::*;
    use Operation::*;
    let operand = match arg {
        Constant(value) => value,
        Old => old,
    };

    match op {
        Multiply => old * operand,
        Add => old + operand,
    }
}

fn play_round(monkeys: &mut [Monkey], inspections: &mut [usize]) {
    for idx in 0..monkeys.len() {
        let monkey = monkeys[idx].clone();
        monkeys[idx].items.truncate(0);
        inspections[idx] += monkey.items.len();

        monkey
            .items
            .into_iter()
            .map(|item| {
                let item = process(item, monkey.operation) / 3;
                if item % monkey.test == 0 {
                    (item, monkey.throw_if_true)
                } else {
                    (item, monkey.throw_if_false)
                }
            })
            .for_each(|(item, to)| monkeys[to].items.push(item))
    }
}

fn main() -> Result<()> {
    let mut monkeys = read(BufReader::new(stdin()))?;
    let mut inspections = vec![0; monkeys.len()];

    (0..20).for_each(|_| play_round(&mut monkeys, &mut inspections));
    inspections.sort_unstable_by_key(|&val| Reverse(val));
    let level_of_monkey_business: usize = inspections[..2].iter().copied().product();

    println!(
        "Level of monkey business after 20 rounds is {}",
        level_of_monkey_business
    );
    Ok(())
}
