use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    io::{stdin, BufReader, Read},
};

#[derive(Debug, Clone, Copy)]
enum RockFormation {
    Horizontal,
    Plus,
    InverseL,
    Vertical,
    Square,
}

impl RockFormation {
    const ORDER: &[RockFormation] = &[
        RockFormation::Horizontal,
        RockFormation::Plus,
        RockFormation::InverseL,
        RockFormation::Vertical,
        RockFormation::Square,
    ];

    fn pieces(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        use RockFormation::*;

        match self {
            Horizontal => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Plus => vec![
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
            ],
            InverseL => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Vertical => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
        .into_iter()
    }

    fn place(&self, (x, y): (usize, usize), map: &mut [Vec<bool>]) {
        self.pieces((x, y)).for_each(|(px, py)| {
            map[py][px] = true;
        });
    }

    fn collides(&self, (x, y): (usize, usize), map: &[Vec<bool>]) -> bool {
        self.pieces((x, y)).any(|(px, py)| map[py][px])
    }

    fn width(&self) -> usize {
        use RockFormation::*;
        match self {
            Horizontal => 4,
            Plus => 3,
            InverseL => 3,
            Vertical => 1,
            Square => 2,
        }
    }

    fn height(&self) -> usize {
        use RockFormation::*;

        match self {
            Horizontal => 1,
            Plus => 3,
            InverseL => 3,
            Vertical => 4,
            Square => 2,
        }
    }

    fn move_stream(&self, (x, y): (usize, usize), pattern: JetPattern, map: &[Vec<bool>]) -> usize {
        let width = self.width();

        use JetPattern::*;
        let new_x = match pattern {
            Left => x.saturating_sub(1),
            Right => (x + 1).clamp(0, 7 - width),
        };

        if self.collides((new_x, y), map) {
            x
        } else {
            new_x
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum JetPattern {
    Left,
    Right,
}

fn drop_block(
    blocks: &mut impl Iterator<Item = (usize, RockFormation)>,
    streams: &mut impl Iterator<Item = (usize, JetPattern)>,
    map: &mut [Vec<bool>],
    drop_height: usize,
) -> (usize, usize, usize) {
    let mut pos = (2, drop_height + 3);
    let (block_idx, block) = blocks.next().expect("infinite iterator");

    let jet_idx = loop {
        let (jet_idx, jet_pattern) = streams.next().expect("infinite iterator");
        pos.0 = block.move_stream(pos, jet_pattern, map);

        if block.collides((pos.0, pos.1 - 1), map) {
            block.place(pos, map);
            break jet_idx;
        } else {
            pos.1 -= 1;
        }
    };

    (block_idx, jet_idx, drop_height.max(pos.1 + block.height()))
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct State([usize; 7], usize, usize);

impl State {
    fn ceiling_map(map: &[Vec<bool>], height: usize) -> [usize; 7] {
        let mut result = [0; 7];
        for (idx, h) in (0..7)
            .map(|x| {
                let mut height = height;
                let mut count = 0;
                while !map[height][x] {
                    height -= 1;
                    count += 1;
                }

                count
            })
            .enumerate()
        {
            result[idx] = h;
        }

        result
    }
}

fn simulate_up_to(
    target: usize,
    mut stream_cycle: &mut impl Iterator<Item = (usize, JetPattern)>,
    mut block_cycle: &mut impl Iterator<Item = (usize, RockFormation)>,
) -> usize {
    let upper_bound = 500000;
    let mut map = vec![vec![false; 7]; upper_bound * 4];
    (0..7).for_each(|x| map[0][x] = true);
    let mut cache: HashMap<State, (usize, usize)> = HashMap::new();

    let mut height = 1;
    for block_no in 0..upper_bound {
        let (block_idx, jet_idx, height_now) =
            drop_block(&mut block_cycle, &mut stream_cycle, &mut map, height);
        height = height_now;

        let ceiling = State::ceiling_map(&map, height);
        let state = State(ceiling, block_idx, jet_idx);

        #[allow(clippy::map_entry)]
        if cache.contains_key(&state) {
            let (blocks_placed_before, height_before) = cache.get(&state).copied().unwrap();
            let how_many_blocks = block_no + 1 - blocks_placed_before;
            let height_diff = height - height_before;
            let repeats = (target - (block_no + 1)) / how_many_blocks;
            let remainder_count = (target - (block_no + 1)) - (repeats * how_many_blocks);
            let mut total_height = height_diff * repeats;

            (0..remainder_count).for_each(|_| {
                let (_, _, height_now) =
                    drop_block(&mut block_cycle, &mut stream_cycle, &mut map, height);
                height = height_now;
            });

            total_height += height;
            return total_height - 1;
        } else {
            cache.insert(state, (block_no + 1, height));
        }
    }

    height
}

fn main() -> Result<()> {
    let mut buf = String::new();
    BufReader::new(stdin()).read_to_string(&mut buf)?;
    let streams = buf
        .trim()
        .chars()
        .map(|ch| match ch {
            '<' => Ok(JetPattern::Left),
            '>' => Ok(JetPattern::Right),
            _ => Err(anyhow!("failed to read jet stream - unknown char: {}", ch)),
        })
        .collect::<Result<Vec<_>>>()?;
    let mut map = vec![vec![false; 7]; 8100];
    (0..7).for_each(|x| map[0][x] = true);
    let mut block_cycle = RockFormation::ORDER.iter().copied().enumerate().cycle();
    let mut stream_cycle = streams.iter().copied().enumerate().cycle();

    let height = simulate_up_to(2022, &mut stream_cycle, &mut block_cycle);
    println!("new alg 2022 = {}", height);

    let mut map = vec![vec![false; 7]; 8100];
    (0..7).for_each(|x| map[0][x] = true);
    let mut block_cycle = RockFormation::ORDER.iter().copied().enumerate().cycle();
    let mut stream_cycle = streams.iter().copied().enumerate().cycle();
    let height = simulate_up_to(1_000_000_000_000, &mut stream_cycle, &mut block_cycle);
    println!("new alg 1000000000 = {}", height);

    Ok(())
}
