use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(_input: &str) -> Option<u32> {
    Some((254032..=789860).filter(|&n| is_valid_part_one(n)).count() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some((254032..=789860).filter(|&n| is_valid_part_two(n)).count() as u32)
}

fn is_valid_part_one(code: u32) -> bool {
    let mut adjacent_same = false;
    code.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .tuple_windows()
        .all(|(a, b)| {
            adjacent_same |= a == b;
            a <= b
        })
        && adjacent_same
}

fn is_valid_part_two(code: u32) -> bool {
    let mut adjacent_same: Option<u32> = None;
    let mut invalid_doubles: HashSet<u32> = HashSet::new();
    code.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .tuple_windows()
        .all(|(a, b)| {
            if a == b && !invalid_doubles.contains(&a) {
                if let Some(same) = adjacent_same {
                    if same == a {
                        adjacent_same = None;
                        invalid_doubles.insert(a);
                    }
                } else {
                    adjacent_same = Some(a);
                }
            }
            a <= b
        })
        && adjacent_same.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(111111, true)]
    #[case(223450, false)]
    #[case(123789, false)]
    fn test_part_one(#[case] code: u32, #[case] expected: bool) {
        assert_eq!(is_valid_part_one(code), expected);
    }

    #[rstest]
    #[case(112233, true)]
    #[case(123444, false)]
    #[case(111122, true)]
    fn test_part_two(#[case] code: u32, #[case] expected: bool) {
        assert_eq!(is_valid_part_two(code), expected);
    }
}
