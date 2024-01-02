use advent_of_code::intcode_computer::cpu::Cpu;
use pathfinding::grid::Grid;
use std::collections::HashMap;
use tokio::sync::mpsc;

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let program: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
        let (tx, mut rx) = mpsc::channel(32);
        let (mut cpu, cpu_input) = Cpu::new_async(program, None);

        let mut seen: HashMap<(i32, i32), i64> = HashMap::new();
        let mut pos: (i32, i32) = (0, 0);
        let mut dir: isize = 0;

        let cpu_handle = tokio::spawn(async move {
            cpu.run_async(Some(tx)).await;
        });

        cpu_input.send(*seen.entry(pos).or_default()).await.unwrap();
        while let Some(color) = rx.recv().await {
            seen.insert(pos, color);
            let turn = rx.recv().await.unwrap();
            match turn {
                0 => {
                    dir -= 1;
                    if dir < 0 {
                        dir += 4;
                    }
                }
                1 => {
                    dir += 1;
                    dir %= 4;
                }
                _ => unreachable!(),
            }
            pos.0 += DIRECTIONS[dir as usize].0;
            pos.1 += DIRECTIONS[dir as usize].1;
            let send_res = cpu_input.send(*seen.entry(pos).or_default()).await;

            if send_res.is_err() {
                break;
            }
        }
        drop(cpu_handle);
        Some(seen.keys().count() as u32)
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let program: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
        let (tx, mut rx) = mpsc::channel(32);
        let (mut cpu, cpu_input) = Cpu::new_async(program, None);

        let mut seen: HashMap<(i32, i32), i64> = HashMap::new();
        seen.insert((0, 0), 1);
        let mut pos: (i32, i32) = (0, 0);
        let mut dir: isize = 0;

        let cpu_handle = tokio::spawn(async move {
            cpu.run_async(Some(tx)).await;
        });

        cpu_input.send(*seen.entry(pos).or_default()).await.unwrap();
        while let Some(color) = rx.recv().await {
            seen.insert(pos, color);
            let turn = rx.recv().await.unwrap();
            match turn {
                0 => {
                    dir -= 1;
                    if dir < 0 {
                        dir += 4;
                    }
                }
                1 => {
                    dir += 1;
                    dir %= 4;
                }
                _ => unreachable!(),
            }
            pos.0 += DIRECTIONS[dir as usize].0;
            pos.1 += DIRECTIONS[dir as usize].1;
            let send_res = cpu_input.send(*seen.entry(pos).or_default()).await;

            if send_res.is_err() {
                break;
            }
        }
        let whites: Vec<_> = seen
            .into_iter()
            .filter_map(|(pos, color)| if color == 1 { Some(pos) } else { None })
            .collect();
        let grid = Grid::from_coordinates(&whites).unwrap();
        println!("{grid:#?}");
        drop(cpu_handle);
        Some(0)
    })
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
