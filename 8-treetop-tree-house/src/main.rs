use anyhow::{anyhow, Error, Result};
use std::{
    io::{stdin, BufReader, Read},
    str::FromStr,
};

#[derive(Debug)]
struct TreeMap {
    width: usize,
    height: usize,
    trees: Vec<Vec<u8>>,
}

impl FromStr for TreeMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|digit| {
                        digit
                            .to_digit(10)
                            .map(|v| v as u8)
                            .ok_or_else(|| anyhow!("failed to parse digit {}", digit))
                    })
                    .collect::<Result<Vec<u8>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        if trees.is_empty() {
            return Err(anyhow!("tree map is empty"));
        }

        let (width, height) = (trees[0].len(), trees.len());

        Ok(Self {
            trees,
            width,
            height,
        })
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct OcclusionPoint {
    top: u8,
    left: u8,
    right: u8,
    bottom: u8,
}

struct OcclusionMap(Vec<Vec<OcclusionPoint>>);

impl OcclusionMap {
    fn build(map: &TreeMap) -> Self {
        let mut result = vec![vec![OcclusionPoint::default(); map.width]; map.height];

        for h in 0..map.height {
            for w in 0..map.width {
                if w > 0 {
                    result[h][w].left = result[h][w - 1].left.max(map.trees[h][w]);
                } else {
                    result[h][w].left = map.trees[h][w];
                }

                if h > 0 {
                    result[h][w].top = result[h - 1][w].top.max(map.trees[h][w]);
                } else {
                    result[h][w].top = map.trees[h][w];
                }

                let bh = map.height - h - 1;
                if bh + 1 < map.height {
                    result[bh][w].bottom = result[bh + 1][w].bottom.max(map.trees[bh][w]);
                } else {
                    result[bh][w].bottom = map.trees[bh][w];
                }

                let bw = map.width - w - 1;
                if bw + 1 < map.width {
                    result[h][bw].right = result[h][bw + 1].right.max(map.trees[h][bw]);
                } else {
                    result[h][bw].right = map.trees[h][bw];
                }
            }
        }

        Self(result)
    }

    fn is_visible(&self, tree_map: &TreeMap, &(x, y): &(usize, usize)) -> bool {
        if x == 0 || y == 0 || y + 1 == tree_map.height || x + 1 == tree_map.width {
            true
        } else {
            let top = self.0[y - 1][x].top;
            let bottom = self.0[y + 1][x].bottom;
            let left = self.0[y][x - 1].left;
            let right = self.0[y][x + 1].right;

            top < tree_map.trees[y][x]
                || bottom < tree_map.trees[y][x]
                || left < tree_map.trees[y][x]
                || right < tree_map.trees[y][x]
        }
    }
}

impl TreeMap {
    fn scenic_score(&self, (x, y): (usize, usize)) -> usize {
        let length = self.trees[y][x];

        let left = (0..x)
            .rev()
            .take_while(|x| self.trees[y][*x] < length)
            .count();

        let right = (x + 1..self.width)
            .take_while(|x| self.trees[y][*x] < length)
            .count();

        let top = (0..y)
            .rev()
            .take_while(|y| self.trees[*y][x] < length)
            .count();

        let bottom = (y + 1..self.height)
            .take_while(|y| self.trees[*y][x] < length)
            .count();

        (left + (x - left != 0) as usize)
            * (right + (x + right + 1 != self.width) as usize)
            * (top + (y - top != 0) as usize)
            * (bottom + (y + bottom + 1 != self.height) as usize)
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    BufReader::new(stdin()).read_to_string(&mut input)?;
    let input: TreeMap = input.parse()?;
    let occlusion_map = OcclusionMap::build(&input);

    let visible_trees = (0..input.height)
        .into_iter()
        .flat_map(|y| (0..input.width).into_iter().map(move |x| (x, y)))
        .filter(|pt| occlusion_map.is_visible(&input, pt))
        .count();

     let maximum_scenic_score = (0..input.height)
         .into_iter()
         .flat_map(|y| (0..input.width).into_iter().map(move |x| (x, y)))
         .map(|pt| input.scenic_score(pt))
         .max()
         .ok_or_else(|| anyhow!("empty input"))?;


    println!("{} trees are visible", visible_trees);
    println!("{} is maximum scenic score", maximum_scenic_score);

    Ok(())
}
