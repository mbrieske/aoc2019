use std::collections::HashMap;

use advent_of_code::intcode_computer::cpu::Cpu;
use itertools::Itertools;
use tokio::sync::mpsc;

advent_of_code::solution!(13);

enum Tile {
    Empty,  // 0 is an empty tile. No game object appears in this tile.
    Wall,   // 1 is a wall tile. Walls are indestructible barriers.
    Block,  // 2 is a block tile. Blocks can be broken by the ball.
    Paddle, // 3 is a horizontal paddle tile. The paddle is indestructible.
    Ball,   // 4 is a ball tile. The ball moves diagonally and bounces off objects.
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let program: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cpu = Cpu::new(program);
    cpu.run(None);

    let mut grid: HashMap<(i64, i64), Tile> = HashMap::new();
    cpu.outputs
        .iter()
        .tuples::<(&i64, &i64, &i64)>()
        .for_each(|(&x, &y, &t)| {
            grid.insert((x, y), Tile::from(t));
        });
    Some(
        grid.into_values()
            .filter(|t| matches!(t, Tile::Block))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), None)]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file("examples", DAY), None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
