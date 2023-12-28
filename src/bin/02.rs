use advent_of_code::intcode_computer::cpu::Cpu;
use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut program: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    program[1] = 12;
    program[2] = 2;
    let mut cpu = Cpu::new(program);
    cpu.run();
    Some(cpu.program[0] as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let program: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let solution = (0..=99)
        .cartesian_product(0..=99)
        .find_map(|(noun, verb)| {
            let mut p = program.clone();
            p[1] = noun;
            p[2] = verb;
            let mut cpu = Cpu::new(p);
            cpu.run();
            if cpu.program[0] == 19690720 {
                Some(100 * noun + verb)
            } else {
                None
            }
        })
        .unwrap() as u32;
    Some(solution)
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
