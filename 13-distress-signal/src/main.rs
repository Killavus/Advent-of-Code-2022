use anyhow::Result;

#[derive(Debug, Clone)]
enum PacketContent {
    Integer(i64),
    Sublist(Vec<PacketContent>),
}

#[derive(Debug, Clone)]
struct Packet {
    list: Vec<PacketContent>,
}

#[derive(Debug, Clone)]
struct PacketPair {
    first: Packet,
    second: Packet,
}

use std::{
    cmp::Ordering,
    fmt::Display,
    io::{stdin, BufRead, BufReader},
};

use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn parse_integer(i: &str) -> IResult<&str, i64> {
    i64(i)
}

fn parse_list(i: &str) -> IResult<&str, Vec<PacketContent>> {
    use nom::Parser;

    delimited(
        tag("["),
        separated_list0(
            tag(","),
            map(parse_integer, PacketContent::Integer).or(map(parse_list, PacketContent::Sublist)),
        ),
        tag("]"),
    )(i)
}

fn parse_packet(i: &str) -> IResult<&str, Packet> {
    map(parse_list, |contents| Packet { list: contents })(i)
}

fn parse_packet_pair(i: &str) -> IResult<&str, PacketPair> {
    map(
        terminated(
            separated_pair(parse_packet, line_ending, parse_packet),
            line_ending,
        ),
        |(first, second)| PacketPair { first, second },
    )(i)
}

fn read(mut reader: impl BufRead) -> Result<Vec<PacketPair>> {
    use anyhow::anyhow;
    let mut buf = String::new();
    let mut result = vec![];
    let mut line_count = 0;

    loop {
        if reader.read_line(&mut buf)? == 0 {
            break;
        };
        line_count += 1;

        if line_count == 3 {
            buf.truncate(0);
            line_count = 0;
        }

        if line_count == 2 {
            result.push(
                all_consuming(parse_packet_pair)(&buf)
                    .map_err(|err| anyhow!("failed to parse packet pair: {}", err))
                    .map(|(_, pair)| pair)?,
            )
        }
    }

    Ok(result)
}

impl Packet {
    fn into_sublist(self) -> PacketContent {
        PacketContent::Sublist(self.list)
    }

    fn in_right_order(&self, other: &Self) -> Option<bool> {
        PacketContent::Sublist(self.list.clone()).in_right_order(&other.clone().into_sublist())
    }

    fn is_divider_packet(&self) -> bool {
        if self.list.len() == 1 {
            match &self.list[0] {
                PacketContent::Sublist(s) => {
                    s.len() == 1 && matches!(s[0], PacketContent::Integer(2 | 6))
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

impl Display for PacketContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketContent::Integer(i) => {
                write!(f, "{}", i)?;
            }
            PacketContent::Sublist(s) => {
                write!(f, "[")?;
                for (idx, i) in s.iter().enumerate() {
                    write!(f, "{}", i)?;
                    if idx + 1 != s.len() {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
            }
        }

        Ok(())
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone().into_sublist())
    }
}

impl PacketContent {
    fn in_right_order(&self, other: &Self) -> Option<bool> {
        use PacketContent::*;

        match (self, other) {
            (Integer(i), Integer(i2)) => {
                if i == i2 {
                    None
                } else {
                    Some(i < i2)
                }
            }
            (Sublist(s1), Sublist(s2)) => Self::sublists_in_right_order(s1, s2),
            (Integer(i), Sublist(v2)) => {
                Self::sublists_in_right_order(&[PacketContent::Integer(*i)], v2)
            }
            (Sublist(s1), Integer(i)) => {
                Self::sublists_in_right_order(s1, &[PacketContent::Integer(*i)])
            }
        }
    }

    fn sublists_in_right_order(left: &[PacketContent], right: &[PacketContent]) -> Option<bool> {
        let mut left_iter = left.iter();
        let mut right_iter = right.iter();

        loop {
            let left = left_iter.next();
            let right = right_iter.next();

            if left.is_none() && right.is_some() {
                return Some(true);
            }

            if left.is_some() && right.is_none() {
                return Some(false);
            }

            if left.is_none() && right.is_none() {
                return None;
            }

            let left = left.unwrap();
            let right = right.unwrap();

            if let Some(value) = left.in_right_order(right) {
                return Some(value);
            }
        }
    }
}

impl PacketPair {
    fn in_right_order(&self) -> Option<bool> {
        self.first.in_right_order(&self.second)
    }

    fn into_slice(self) -> [Packet; 2] {
        [self.first, self.second]
    }
}

fn main() -> Result<()> {
    let packet_pairs = read(BufReader::new(stdin()))?;
    let sum_of_right_order_indices = packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.in_right_order().unwrap_or(false))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!(
        "Sum of indices of right order packet pairs is {}",
        sum_of_right_order_indices
    );

    let mut packets = packet_pairs
        .into_iter()
        .flat_map(|pair| pair.into_slice())
        .collect::<Vec<_>>();
    packets.push(Packet {
        list: vec![PacketContent::Sublist(vec![PacketContent::Integer(2)])],
    });
    packets.push(Packet {
        list: vec![PacketContent::Sublist(vec![PacketContent::Integer(6)])],
    });

    packets.sort_by(|p1, p2| match p1.in_right_order(p2) {
        None => Ordering::Equal,
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
    });

    packets.iter().enumerate().for_each(|(idx, x)| {
        println!("{} {}", idx + 1, x);
    });

    let decoder_key: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, p)| p.is_divider_packet())
        .map(|(i, _)| i + 1)
        .product();

    println!("Decoder key for the distress signal is {}", decoder_key);

    Ok(())
}
