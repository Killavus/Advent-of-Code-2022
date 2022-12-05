use std::{
    io::BufReader,
    io::{stdin, BufRead},
};

#[derive(Debug, Clone)]
struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

#[derive(Clone, Copy)]
struct CraneMove {
    count: usize,
    source: usize,
    target: usize,
}

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::streaming::tag,
    bytes::streaming::take,
    character::streaming::{anychar, digit1, line_ending, space1},
    combinator::{eof, map, map_parser, map_res, opt},
    multi::{many1, many_till, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

impl CrateStacks {
    fn message(&self) -> String {
        self.stacks.iter().flat_map(|stack| stack.last()).collect()
    }
}

fn parse_command(line: &str) -> IResult<&str, CraneMove> {
    map(
        terminated(
            tuple((
                map_res(preceded(tuple((tag("move"), space1)), digit1), str::parse),
                map_res(
                    preceded(tuple((space1, tag("from"), space1)), digit1),
                    str::parse,
                ),
                map_res(
                    preceded(tuple((space1, tag("to"), space1)), digit1),
                    str::parse,
                ),
            )),
            alt((line_ending, eof)),
        ),
        |(count, source, target)| CraneMove {
            count,
            source,
            target,
        },
    )(line)
}

fn parse_crate(line: &str) -> IResult<&str, Option<char>> {
    map_parser(take(3usize), opt(delimited(tag("["), anychar, tag("]"))))(line)
}

fn parse_crate_line(line: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(
        separated_list0(tag(" "), parse_crate),
        alt((line_ending, eof)),
    )(line)
}

fn parse_crate_numbering(line: &str) -> IResult<&str, ()> {
    map(
        terminated(
            many1(delimited(opt(space1), digit1, space1)),
            alt((line_ending, eof)),
        ),
        |_| (),
    )(line)
}

fn parse_crate_stacks(input: &str) -> IResult<&str, CrateStacks> {
    map(
        many_till(parse_crate_line, parse_crate_numbering),
        |(stacks, _)| {
            let stack_size = stacks.len();

            let mut parsed = vec![Vec::with_capacity(stack_size); stacks[stack_size - 1].len()];

            for slice in stacks.into_iter().rev() {
                for (idx, content) in slice.into_iter().enumerate() {
                    if let Some(c) = content {
                        parsed[idx].push(c);
                    }
                }
            }

            CrateStacks { stacks: parsed }
        },
    )(input)
}

fn read_stacks(mut reader: impl BufRead) -> Result<(impl BufRead, CrateStacks)> {
    let mut buf = String::new();

    while let Ok(num_bytes) = reader.read_line(&mut buf) {
        if num_bytes == 0 {
            return Err(anyhow!("eof reached while reading stacks"));
        }

        let parsed = parse_crate_stacks(&buf);

        match parsed {
            Ok((rest, stacks)) => {
                if !rest.is_empty() {
                    return Err(anyhow!("data lingering"));
                }

                return Ok((reader, stacks));
            }
            Err(e) => {
                if e.is_incomplete() {
                    continue;
                } else {
                    return Err(e).map_err(|e| anyhow!("failed to parse input: {}", e));
                }
            }
        }
    }

    Err(anyhow!("input is invalid"))
}

fn commands(mut reader: impl BufRead + 'static) -> impl Iterator<Item = Result<CraneMove>> {
    let mut buf = String::new();
    // Remove newline between stacks and commands.
    reader.read_line(&mut buf).ok();

    std::iter::from_fn(move || {
        buf.truncate(0);
        match reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(
                parse_command(&buf)
                    .map(|(_, op)| op)
                    .map_err(|e| anyhow!("failed to parse command: {}", e)),
            ),
            Err(_) => None,
        }
    })
}

impl CraneMove {
    fn execute(&self, CrateStacks { stacks }: &mut CrateStacks, new_version: bool) {
        let Self {
            count,
            source,
            target,
        } = self;

        let split_at = stacks[*source - 1].len() - count;
        let to_move = stacks[*source - 1].split_off(split_at);
        if new_version {
            stacks[*target - 1].extend(to_move);
        } else {
            stacks[*target - 1].extend(to_move.into_iter().rev());
        }
    }
}

fn main() -> Result<()> {
    let (reader, mut stacks) = read_stacks(BufReader::new(stdin()))?;

    let mut new_version_stacks = stacks.clone();

    for command in commands(reader) {
        let command = command?;

        command.execute(&mut stacks, false);
        command.execute(&mut new_version_stacks, true);
    }

    println!(
        "Crates on top of stacks create a message {}",
        stacks.message()
    );

    println!(
        "Crates on top of stacks using new crane create a message {}",
        new_version_stacks.message()
    );

    Ok(())
}
