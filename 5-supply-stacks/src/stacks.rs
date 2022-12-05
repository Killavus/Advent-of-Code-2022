use nom::{
    branch::alt,
    bytes::streaming::{tag, take},
    character::streaming::{anychar, digit1, line_ending, space1},
    combinator::{eof, map, map_parser, opt},
    multi::{many1, many_till, separated_list0},
    sequence::{delimited, terminated},
    IResult,
};

use anyhow::{anyhow, Result};

use std::io::BufRead;

#[derive(Debug, Clone)]
pub struct CrateStacks {
    pub stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    pub fn message(&self) -> String {
        self.stacks.iter().flat_map(|stack| stack.last()).collect()
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(many_till(stack_level, stack_numbering), |(stacks, _)| {
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
        })(input)
    }

    pub fn read(mut reader: impl BufRead) -> Result<(impl BufRead, Self)> {
        let mut buf = String::new();

        while let Ok(num_bytes) = reader.read_line(&mut buf) {
            if num_bytes == 0 {
                break;
            }

            let parsed = Self::parse(&buf);

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
}

fn stack_crate(line: &str) -> IResult<&str, Option<char>> {
    map_parser(take(3usize), opt(delimited(tag("["), anychar, tag("]"))))(line)
}

fn stack_level(line: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(
        separated_list0(tag(" "), stack_crate),
        alt((line_ending, eof)),
    )(line)
}

fn stack_numbering(line: &str) -> IResult<&str, ()> {
    map(
        terminated(
            many1(delimited(opt(space1), digit1, space1)),
            alt((line_ending, eof)),
        ),
        |_| (),
    )(line)
}
