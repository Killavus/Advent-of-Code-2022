use std::io::{prelude::*, stdin, BufReader};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Eq, PartialEq, Copy, Clone)]
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
        if &opponent.win_move() == self {
            6
        } else if &opponent.lose_move() == self {
            0
        } else {
            3
        }
    }

    fn win_move(&self) -> Self {
        use Move::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn lose_move(&self) -> Self {
        use Move::*;

        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn interpret(input: impl BufRead) -> AppResult<(usize, usize)> {
    use Move::*;
    use Outcome::*;

    let mut total_score_by_reasoning = 0;
    let mut total_score_correctly = 0;

    for line in input.lines() {
        let line = line?;
        let mut line = line.split_ascii_whitespace();
        let opponent_move = match line.next().expect("valid input") {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("invalid input"),
        };

        let second_column = line.next().expect("valid input");

        let your_move_by_reasoning = match second_column {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("invalid input"),
        };

        let outcome = match second_column {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("invalid input"),
        };

        let your_correct_move = match outcome {
            Win => opponent_move.win_move(),
            Lose => opponent_move.lose_move(),
            _ => opponent_move,
        };

        total_score_by_reasoning += your_move_by_reasoning.play(&opponent_move);
        total_score_correctly += your_correct_move.play(&opponent_move);
    }

    Ok((total_score_by_reasoning, total_score_correctly))
}

fn main() -> AppResult<()> {
    let (total_score_by_reasoning, total_score_correctly) = interpret(BufReader::new(stdin()))?;

    println!(
        "By reasoning, you should be able to get {} points.",
        total_score_by_reasoning
    );

    println!(
        "By reading strategy guide correctly, you will get {} points.",
        total_score_correctly
    );

    Ok(())
}
