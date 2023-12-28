use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let wires: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|instruction| {
                    let (dir, dist) = instruction.split_at(1);
                    let dist: i32 = dist.parse().unwrap();
                    match dir.chars().next().unwrap() {
                        'U' => (0, -dist),
                        'D' => (0, dist),
                        'L' => (-dist, 0),
                        'R' => (dist, 0),
                        _ => unreachable!(),
                    }
                })
                .scan((0, 0), |state, (dx, dy)| {
                    *state = (state.0 + dx, state.1 + dy);
                    Some(*state)
                })
                .collect()
        })
        .collect();

    let intersections: Vec<(i32, i32)> = Vec::new();

    wires
        .get(0).unwrap().windows(2)
        .cartesian_product(wires.get(1).unwrap().windows(2))
        .for_each(|([wire_start, wire_end], [other_start, other_end])| {
            if wire.0 <= x2_end && x1_end >= x2_start
        });

    todo!()
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
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(6))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(159))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 3), Some(135))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(6))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(159))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 3), Some(135))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
