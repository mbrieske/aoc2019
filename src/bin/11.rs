use advent_of_code::intcode_computer::cpu::{Cpu, Msg};
use pathfinding::grid::Grid;
use std::collections::HashMap;
use tokio::sync::mpsc;

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

advent_of_code::solution!(11, 1);

pub fn part_one(input: &str) -> Option<u32> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let program: Vec<_> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (tx, mut rx) = mpsc::channel(32);
        let (mut cpu, cpu_input) = Cpu::new_async(program);

        let mut seen: HashMap<(i32, i32), i64> = HashMap::new();
        let mut pos: (i32, i32) = (0, 0);
        let mut dir: isize = 0;

        let cpu_handle = tokio::spawn(async move {
            cpu.run_async(tx).await;
        });

        cpu_input
            .send(Msg::Value(*seen.entry(pos).or_default()))
            .await
            .unwrap();
        loop {
            let recv = rx.recv().await;
            if let Some(recv) = recv {
                if let Msg::Value(color) = recv {
                    seen.insert(pos, color);
                    let turn = rx.recv().await.unwrap();
                    match turn {
                        Msg::Value(0) => {
                            dir -= 1;
                            if dir < 0 {
                                dir += 4;
                            }
                        }
                        Msg::Value(1) => {
                            dir += 1;
                            dir %= 4;
                        }
                        _ => unreachable!(),
                    }
                    pos.0 += DIRECTIONS[dir as usize].0;
                    pos.1 += DIRECTIONS[dir as usize].1;
                    let send_res = cpu_input
                        .send(Msg::Value(*seen.entry(pos).or_default()))
                        .await;

                    if send_res.is_err() {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        cpu_handle.await.unwrap();
        Some(seen.keys().count() as u32)
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let program: Vec<_> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (tx, mut rx) = mpsc::channel(32);
        let (mut cpu, cpu_input) = Cpu::new_async(program);

        let mut seen: HashMap<(i32, i32), i64> = HashMap::new();
        seen.insert((0, 0), 1);
        let mut pos: (i32, i32) = (0, 0);
        let mut dir: isize = 0;

        let cpu_handle = tokio::spawn(async move {
            cpu.run_async(tx).await;
        });

        cpu_input
            .send(Msg::Value(*seen.entry(pos).or_default()))
            .await
            .unwrap();
        loop {
            let recv = rx.recv().await;
            if let Some(recv) = recv {
                if let Msg::Value(color) = recv {
                    seen.insert(pos, color);
                    let turn = rx.recv().await.unwrap();
                    match turn {
                        Msg::Value(0) => {
                            dir -= 1;
                            if dir < 0 {
                                dir += 4;
                            }
                        }
                        Msg::Value(1) => {
                            dir += 1;
                            dir %= 4;
                        }
                        _ => unreachable!(),
                    }
                    pos.0 += DIRECTIONS[dir as usize].0;
                    pos.1 += DIRECTIONS[dir as usize].1;
                    let send_res = cpu_input
                        .send(Msg::Value(*seen.entry(pos).or_default()))
                        .await;

                    if send_res.is_err() {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        let whites: Vec<_> = seen
            .into_iter()
            .filter_map(|(pos, color)| if color == 1 { Some(pos) } else { None })
            .collect();
        let grid = Grid::from_coordinates(&whites).unwrap();
        println!("{grid:#?}");
        cpu_handle.await.unwrap();
        Some(0)
    })
}
