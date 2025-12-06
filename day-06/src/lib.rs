use shared::{
    parse::{Parsable, ParsableNonWhitespaceByte},
    *,
};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let mut lines = _input.lines().map(|line| line.bytes()).collect::<Vec<_>>();
    let mut instructions = lines.pop().unwrap();
    let mut solution: usize = 0;

    loop {
        if let Some(instruction) = instructions.next_non_whitespace_byte() {
            match instruction {
                b'*' => {
                    solution += lines
                        .iter_mut()
                        .filter_map(|line| line.next_number())
                        .fold(1, |acc, x: usize| acc * x);
                }
                b'+' => {
                    solution += lines
                        .iter_mut()
                        .filter_map(|line| line.next_number())
                        .fold(0, |acc, x: usize| acc + x);
                }
                _ => {}
            }
        } else {
            break;
        }
    }
    solution.into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 4277556)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(5873191732773)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let mut lines: Vec<std::str::Bytes<'_>> = _input.lines().map(|line| line.bytes()).collect();
    let mut instructions = lines.pop().unwrap();
    let mut solution: usize = 0;

    loop {
        if let Some(instruction) = instructions.next_non_whitespace_byte() {
            let mut sum: usize = 0;
            if instruction == b'*' {
                sum = 1;
                while let Some(number) = next_column_number(&mut lines) {
                    sum *= number;
                }
            } else {
                while let Some(number) = next_column_number(&mut lines) {
                    sum += number;
                }
            }

            solution += sum;
        } else {
            break;
        }
    }
    solution.into()
}

fn next_column_number(lines: &mut Vec<std::str::Bytes<'_>>) -> Option<usize> {
    let mut number: usize = 0;
    for line in lines.iter_mut() {
        if let Some(next) = line.next()
            && next != b' '
        {
            number = number * 10 + (next - b'0') as usize;
        }
    }

    if number == 0 {
        return None;
    }

    Some(number)
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 3263827)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(11386445308378)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
