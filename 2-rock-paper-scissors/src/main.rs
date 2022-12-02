use std::io::{prelude::*, stdin, BufReader};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn play(&self, opponent: &Self) -> usize {
        self.move_point() + self.match_point(opponent)
    }

    fn move_point(&self) -> usize {
        use Move::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn match_point(&self, opponent: &Self) -> usize {
        use Move::*;

        match (self, opponent) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Paper, Rock) | (Rock, Scissors) | (Scissors, Paper) => 6,
            _ => 0,
        }
    }
}

fn interpret_by_reasoning(input: impl BufRead) -> AppResult<usize> {
    let mut total_score = 0;

    for line in input.lines() {
        use Move::*;

        let line = line?;
        let mut line = line.split_ascii_whitespace();
        let opponent_move = match line.next().expect("valid input") {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("invalid input"),
        };

        let your_move = match line.next().expect("valid input") {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("invalid input"),
        };

        total_score += your_move.play(&opponent_move);
    }

    Ok(total_score)
}

fn main() -> AppResult<()> {
    let total_score_by_reasoning = interpret_by_reasoning(BufReader::new(stdin()))?;

    println!(
        "By reasoning, you should be able to get {} points.",
        total_score_by_reasoning
    );

    Ok(())
}
