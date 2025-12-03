use std::ops::Index;

use shared::{parse::ParsableDigit, *};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| {
            let mut bytes = line.bytes();
            let mut top_1: u8 = 0;
            let mut top_2: u8 = 0;
            let mut next: Option<u8> = None;

            while let Some(number) = bytes.next_digit() {
                if number > top_1 {
                    top_2 = top_1;
                    top_1 = number;
                    next = None;
                } else if next.map_or(true, |n| n < number) {
                    next = Some(number);
                }
            }

            if let Some(n) = next {
                return top_1 as usize * 10 + n as usize;
            } else {
                return top_2 as usize * 10 + top_1 as usize;
            }
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 357)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(17166)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| {
            let mut bytes = line.bytes();
            let mut list: Vec<u8> = Vec::new();

            for _ in 0..12 {
                list.push(bytes.next_digit().unwrap());
            }

            while let Some(number) = bytes.next_digit() {
                list.push(number);

                let mut prev = list.first().unwrap();
                let mut changed = false;
                for i in 1..13 {
                    let next = list.index(i);
                    if next > prev {
                        list.remove(i - 1);
                        changed = true;
                        break;
                    }
                    prev = next;
                }

                if !changed {
                    list.pop();
                }
            }

            let mut sol: usize = 0;
            for n in list {
                sol = sol * 10 + n as usize;
            }
            sol
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 3121910778619)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(169077317650774)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
