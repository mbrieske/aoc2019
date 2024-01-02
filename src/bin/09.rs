use advent_of_code::intcode_computer::cpu::Cpu;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let program: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut cpu = Cpu::new(program);
    cpu.run(Some(vec![1]));
    Some(*cpu.outputs.last().unwrap())
}

pub fn part_two(input: &str) -> Option<i64> {
    let program: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut cpu = Cpu::new(program);
    cpu.run(Some(vec![2]));
    Some(*cpu.outputs.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_quine() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);
        let program: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
        let mut cpu = Cpu::new(program.clone());
        cpu.run(None);
        let memory_after_run: Vec<_> = cpu
            .program
            .into_iter()
            .sorted_by_key(|memory| memory.0)
            .map(|(_, v)| v)
            .take(program.len())
            .collect();
        assert_eq!(memory_after_run, program);
    }

    #[test]
    fn test_large() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let program: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
        let mut cpu = Cpu::new(program.clone());
        cpu.run(None);
        assert_eq!(cpu.outputs.last().unwrap(), &program[1]);
    }

    #[test]
    fn test_relative() {
        let program = vec![109, 19, 204, -34, 99];
        let mut cpu = Cpu::new(program);
        cpu.relative_base = 2000;
        cpu.program.insert(1985, 42);
        cpu.run(None);
        assert_eq!(cpu.outputs.last().unwrap(), &42);
    }
}
