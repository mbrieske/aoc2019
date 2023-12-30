use advent_of_code::intcode_computer::cpu::Cpu;
use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut program: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    program[1] = 12;
    program[2] = 2;
    let mut cpu = Cpu::new(program);
    cpu.run(None);
    Some(cpu.program[0])
}

pub fn part_two(input: &str) -> Option<i32> {
    let program: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let solution = (0..=99)
        .cartesian_product(0..=99)
        .find_map(|(noun, verb)| {
            let mut p = program.clone();
            p[1] = noun;
            p[2] = verb;
            let mut cpu = Cpu::new(p);
            cpu.run(None);
            if cpu.program[0] == 19690720 {
                Some(100 * noun + verb)
            } else {
                None
            }
        })
        .unwrap();
    Some(solution)
}
