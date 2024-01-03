use advent_of_code::intcode_computer::cpu::{Cpu, Msg};
use itertools::Itertools;
use std::cmp::Ordering::*;
use std::collections::HashMap;
use tokio::sync::mpsc;

advent_of_code::solution!(13);

enum Tile {
    Other,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 | 1 => Self::Other,
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
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut program: Vec<_> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        *program.get_mut(0).unwrap() = 2;

        let (mut cpu, cpu_input) = Cpu::new_async(program);

        let (tx, mut rx) = mpsc::channel(32);
        tokio::spawn(async move {
            cpu.run_async(tx).await;
        });

        let mut ball = None;
        let mut paddle = None;
        let mut score = 0;

        loop {
            match rx.recv().await {
                Some(Msg::Value(-1)) => {
                    assert_eq!(rx.recv().await.unwrap(), Msg::Value(0));
                    if let Msg::Value(s) = rx.recv().await.unwrap() {
                        score = s;
                    } else {
                        unreachable!()
                    }
                }
                Some(Msg::Value(x)) => {
                    let y = rx.recv().await.unwrap();
                    let tile = rx.recv().await.unwrap();
                    if let (Msg::Value(_), Msg::Value(tile)) = (y, tile) {
                        match Tile::from(tile) {
                            Tile::Paddle => paddle = Some(x),
                            Tile::Ball => ball = Some(x),
                            _ => (),
                        }
                    } else {
                        unreachable!();
                    }
                }
                Some(Msg::RxRequest) => {
                    let joystick_input = match ball.cmp(&paddle) {
                        Less => -1,
                        Equal => 0,
                        Greater => 1,
                    };
                    cpu_input.send(Msg::Value(joystick_input)).await.unwrap();
                }
                None => break,
            }
        }

        Some(score as u32)
    })
}
