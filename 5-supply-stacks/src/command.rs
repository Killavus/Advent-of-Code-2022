use anyhow::{anyhow, Result};
use std::io::BufRead;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{eof, map, map_res},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Clone, Copy)]
pub struct CraneMove {
    count: usize,
    source: usize,
    target: usize,
}

pub enum CraneVersion {
    CraneMover9000,
    CraneMover9001,
}

impl CraneMove {
    fn parse(line: &str) -> IResult<&str, Self> {
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
            |(count, source, target)| Self {
                count,
                source,
                target,
            },
        )(line)
    }

    pub fn execute(&self, stacks: &mut [Vec<char>], version: CraneVersion) {
        let Self {
            count,
            source,
            target,
        } = self;

        let split_at = stacks[*source - 1].len() - count;
        let to_move = stacks[*source - 1].split_off(split_at);
        match version {
            CraneVersion::CraneMover9000 => {
                stacks[*target - 1].extend(to_move.into_iter().rev());
            }
            CraneVersion::CraneMover9001 => {
                stacks[*target - 1].extend(to_move);
            }
        }
    }
}

pub fn read_all(mut reader: impl BufRead) -> impl Iterator<Item = Result<CraneMove>> {
    let mut buf = String::new();
    // Remove newline between stacks and commands.
    reader.read_line(&mut buf).ok();

    std::iter::from_fn(move || {
        buf.truncate(0);
        match reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(
                CraneMove::parse(&buf)
                    .map(|(_, op)| op)
                    .map_err(|e| anyhow!("failed to parse command: {}", e)),
            ),
            Err(_) => None,
        }
    })
}
