use advent_of_code::intcode_computer::cpu::Cpu;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let program: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut cpu = Cpu::new(program);
    cpu.run();
    None
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
    #[case(&template::read_file("examples", DAY), None)]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&template::read_file("examples", DAY), None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }

    // #[test]
    // fn test_input_output() {
    //     part_one("3,0,4,0,99");
    // }
}
