use std::collections::HashSet;

use shared::math::Digits;
use shared::parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn solve_1(input: &str) -> usize {
    let mut bytes = input.bytes();
    let mut answer: usize = 0;

    while let Some(start) = bytes.next_number() as Option<usize> {
        let end: usize = bytes.next_number().unwrap();
        let start_digits = start.count_digits();
        let end_digits = end.count_digits();

        for digits in start_digits..=end_digits {
            if digits % 2 != 0 {
                continue;
            }

            let factor = 10usize.pow(digits as u32 / 2);
            let from = usize::max(factor / 10, start / factor);
            let to = factor - 1;

            for n in from..=to {
                let x = n + n * factor;
                if x > end {
                    break;
                }
                if x >= start {
                    answer += x;
                }
            }
        }
    }

    answer
}

pub fn part_1(_input: &str) -> Solution {
    solve_1(_input).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 1227775554)]
    #[test_case(include_str!("_manual_1.txt"), 495900)]
    #[test_case(include_str!("_manual_2.txt"), 495495949990950)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(19219508902)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn solve_2(input: &str) -> usize {
    let mut bytes = input.bytes();
    let mut answer: usize = 0;

    let mut map: HashSet<usize> = HashSet::new();

    while let Some(start) = bytes.next_number() as Option<usize> {
        let end: usize = bytes.next_number().unwrap();
        let start_digits = start.count_digits();
        let end_digits = end.count_digits();

        for digits in start_digits..=end_digits {
            let max_window = digits / 2;
            for window in 1..=max_window {
                if digits % window != 0 {
                    continue;
                }

                let div = 10usize.pow((digits - window) as u32);
                let factor = 10usize.pow((window) as u32);
                let from = usize::max(factor / 10, start / div);
                let to = factor - 1;

                for n in from..=to {
                    let mut x = n;
                    for _ in 1..digits / window {
                        x = x * factor + n;
                    }
                    if x > end {
                        break;
                    }
                    if x >= start {
                        if map.insert(x) {
                            answer += x;
                        }
                    }
                }
            }
        }
    }

    answer
}

pub fn part_2(_input: &str) -> Solution {
    solve_2(_input).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 4174379265)]
    #[test_case(include_str!("_manual_3.txt"), 512006)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(27180728081)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
