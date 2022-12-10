#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    AddX(i64),
}

use anyhow::Result;
use std::{
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, space1},
    combinator::{all_consuming, map},
    sequence::separated_pair,
    IResult,
};

impl Instruction {
    fn parse(line: &str) -> IResult<&str, Instruction> {
        all_consuming(alt((
            map(tag("noop"), |_| Instruction::Noop),
            map(separated_pair(tag("addx"), space1, i64), |(_, delta)| {
                Instruction::AddX(delta)
            }),
        )))(line)
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        use anyhow::anyhow;

        match Self::parse(line) {
            Ok((_, instruction)) => Ok(instruction),
            Err(e) => Err(anyhow!("failed to parse instruction: {}", e)),
        }
    }
}

#[derive(Clone, Copy)]
enum CPUState {
    Busy(Instruction, usize),
    Idle,
}

struct Cpu {
    x: i64,
    state: CPUState,
    cycle: usize,
}

impl Instruction {
    fn cycle_length(&self) -> usize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            state: CPUState::Idle,
            cycle: 1,
        }
    }

    fn idle(&self) -> bool {
        matches!(self.state, CPUState::Idle)
    }

    fn tick(&mut self) {
        self.cycle += 1;
        self.update()
    }

    fn current_cycle(&self) -> usize {
        self.cycle
    }

    fn x(&self) -> i64 {
        self.x
    }

    fn load_instruction(&mut self, instruction: Instruction) -> Result<()> {
        use anyhow::anyhow;

        self.idle()
            .then_some(())
            .ok_or_else(|| {
                anyhow!(
                    "trying to load instruction {:?} when CPU is busy",
                    instruction
                )
            })
            .map(|_| {
                self.state = CPUState::Busy(instruction, self.cycle);
            })
    }

    fn addx(&mut self, dx: i64) {
        self.x += dx;
    }

    fn update(&mut self) {
        if let CPUState::Busy(instruction, start) = self.state {
            if self.cycle - start == instruction.cycle_length() {
                self.state = CPUState::Idle;
                if let Instruction::AddX(dx) = instruction {
                    self.addx(dx);
                }
            }
        }
    }
}

fn read(reader: impl BufRead) -> impl Iterator<Item = Result<Instruction>> {
    reader
        .lines()
        .map(|line| line.map_err(Into::into).and_then(|line| line.parse()))
}

fn main() -> Result<()> {
    let mut instructions = read(BufReader::new(stdin()));
    let mut signal_strength = 0;
    let mut cpu = Cpu::new();
    let probing_signal_strength_at = &[20, 60, 100, 140, 180, 220];

    let mut crt_position = 0;

    loop {
        if cpu.idle() {
            if let Some(instruction) = instructions.next() {
                cpu.load_instruction(instruction?)?;
            }
        }

        if (cpu.x() - 1..=cpu.x() + 1).contains(&crt_position) {
            print!("█")
        } else {
            print!(" ")
        }
        cpu.tick();
        crt_position += 1;

        if crt_position > 39 {
            crt_position = 0;
            println!();
        }

        if probing_signal_strength_at.contains(&cpu.current_cycle()) {
            signal_strength += (cpu.current_cycle() as i64) * cpu.x();
        }

        if cpu.current_cycle() == 241 {
            break;
        }
    }

    println!("Sum of signal strengths is {}", signal_strength);
    Ok(())
}
