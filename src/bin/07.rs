use advent_of_code::intcode_computer::cpu::Cpu;
use futures::future::join_all;
use itertools::{enumerate, Itertools};
use tokio::runtime::Runtime;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let program: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let phases = [0, 1, 2, 3, 4];
    let max_signal = phases
        .iter()
        .permutations(phases.len())
        .map(|c| {
            c.into_iter().try_fold(0, |state, phase| {
                run_amplifier(Cpu::new(program.clone()), *phase, state)
            })
        })
        .max()
        .unwrap()
        .unwrap();
    Some(max_signal as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let program: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();

        let phases = [5, 6, 7, 8, 9];
        let output_signals = phases
            .iter()
            .permutations(phases.len())
            .map(|phase_permutation| run_feedback_loop(program.clone(), phase_permutation));
        let output_signals = join_all(output_signals).await;
        output_signals.into_iter().map(|signal| signal as u32).max()
    })
}

fn run_amplifier(mut cpu: Cpu, phase: i64, input: i64) -> Option<i64> {
    cpu.run(Some(vec![phase, input]));
    cpu.outputs.last().copied()
}

async fn run_feedback_loop(program: Vec<i64>, phase_permutation: Vec<&i64>) -> i64 {
    let mut cpus = Vec::new();

    let last_tx = (0..5).fold(None, |last_tx, _| {
        let (cpu, tx) = Cpu::new_async(program.clone(), last_tx);
        cpus.push(cpu);
        Some(tx)
    });
    cpus.first_mut().unwrap().output = last_tx;
    cpus.reverse();

    for (i, cpu) in enumerate(&cpus) {
        cpu.output
            .as_ref()
            .unwrap()
            .send(*phase_permutation[i])
            .await
            .unwrap()
    }

    cpus.last()
        .unwrap()
        .output
        .as_ref()
        .unwrap()
        .send(0)
        .await
        .unwrap();

    let futures: Vec<_> = cpus.iter_mut().map(|cpu| cpu.run_async()).collect();
    join_all(futures).await;
    *cpus.last().unwrap().outputs.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(43210))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(54321))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 3), Some(65210))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u32>) {
        // tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 4), Some(139629729))]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 5), Some(18216))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u32>) {
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
