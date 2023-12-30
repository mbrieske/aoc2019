use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut seen: Vec<HashSet<(i32, i32)>> = vec![HashSet::new(); 2];

    input.lines().enumerate().for_each(|(i, line)| {
        let mut state = (0, 0);
        line.split(',')
            .map(|instruction| {
                let (dir, dist) = instruction.split_at(1);
                (dir, dist.parse::<i32>().unwrap())
            })
            .for_each(|(dir, dist)| {
                let (dx, dy) = match dir {
                    "U" => (0, -dist),
                    "D" => (0, dist),
                    "L" => (-dist, 0),
                    "R" => (dist, 0),
                    _ => unreachable!(),
                };

                match dir {
                    "U" => ((state.1 - dist)..state.1).for_each(|y| {
                        seen[i].insert((state.0, y));
                    }),
                    "D" => ((state.1 + 1)..=state.1 + dist).for_each(|y| {
                        seen[i].insert((state.0, y));
                    }),
                    "L" => ((state.0 - dist)..state.0).for_each(|x| {
                        seen[i].insert((x, state.1));
                    }),
                    "R" => ((state.0 + 1)..=state.0 + dist).for_each(|x| {
                        seen[i].insert((x, state.1));
                    }),
                    _ => unreachable!(),
                };
                state = (state.0 + dx, state.1 + dy);
            });
    });

    let intersections = seen[0].intersection(&seen[1]);

    let min = intersections
        .min_by(|intersection, other| {
            (intersection.0.abs() + intersection.1.abs()).cmp(&(other.0.abs() + other.1.abs()))
        })
        .unwrap();
    Some(min.0.unsigned_abs() + min.1.unsigned_abs())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut seen: Vec<HashMap<(i32, i32), u32>> = vec![HashMap::new(); 2];

    input.lines().enumerate().for_each(|(i, line)| {
        let mut state = (0, 0);
        let mut steps = 0;
        line.split(',')
            .map(|instruction| {
                let (dir, dist) = instruction.split_at(1);
                (dir, dist.parse::<i32>().unwrap())
            })
            .for_each(|(dir, dist)| {
                let (dx, dy) = match dir {
                    "U" => (0, -dist),
                    "D" => (0, dist),
                    "L" => (-dist, 0),
                    "R" => (dist, 0),
                    _ => unreachable!(),
                };

                match dir {
                    "U" => ((state.1 - dist)..state.1).rev().for_each(|y| {
                        steps += 1;
                        seen[i].entry((state.0, y)).or_insert(steps);
                    }),
                    "D" => ((state.1 + 1)..=state.1 + dist).for_each(|y| {
                        steps += 1;
                        seen[i].entry((state.0, y)).or_insert(steps);
                    }),
                    "L" => ((state.0 - dist)..state.0).rev().for_each(|x| {
                        steps += 1;
                        seen[i].entry((x, state.1)).or_insert(steps);
                    }),
                    "R" => ((state.0 + 1)..=state.0 + dist).for_each(|x| {
                        steps += 1;
                        seen[i].entry((x, state.1)).or_insert(steps);
                    }),
                    _ => unreachable!(),
                };
                state = (state.0 + dx, state.1 + dy);
            });
    });

    let keys1: HashSet<(i32, i32)> = seen[0].keys().copied().collect();
    let keys2: HashSet<(i32, i32)> = seen[1].keys().copied().collect();
    let intersections = keys1.intersection(&keys2);

    intersections
        .map(|intersection| seen[0][intersection] + seen[1][intersection])
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(6))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(159))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 3), Some(135))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(30))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(610))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 3), Some(410))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
