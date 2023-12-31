use advent_of_code::intcode_computer::cpu::Cpu;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    run(input, 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    run(input, 5)
}

fn run(program: &str, input: i32) -> Option<u32> {
    let program: Vec<i32> = program.split(',').map(|s| s.parse().unwrap()).collect();
    let mut cpu = Cpu::new(program);
    cpu.run(Some(vec![input]));
    cpu.outputs.last().map(|&output| output as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;

    #[rstest]
    #[case(&template::read_file_part("examples", DAY, 1), None)]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&template::read_file_part("examples", DAY, 2), 8, Some(1))]
    #[case(&template::read_file_part("examples", DAY, 2), 5, Some(0))]
    #[case(&template::read_file_part("examples", DAY, 3), 5, Some(1))]
    #[case(&template::read_file_part("examples", DAY, 3), 9, Some(0))]
    #[case(&template::read_file_part("examples", DAY, 4), 8, Some(1))]
    #[case(&template::read_file_part("examples", DAY, 4), 5, Some(0))]
    #[case(&template::read_file_part("examples", DAY, 5), 5, Some(1))]
    #[case(&template::read_file_part("examples", DAY, 5), 9, Some(0))]
    #[case(&template::read_file_part("examples", DAY, 6), 7, Some(999))]
    #[case(&template::read_file_part("examples", DAY, 6), 8, Some(1000))]
    #[case(&template::read_file_part("examples", DAY, 6), 9, Some(1001))]
    fn test_part_two(#[case] program: &str, #[case] input: i32, #[case] expected: Option<u32>) {
        let result = run(program, input);
        assert_eq!(result, expected);
    }
}
