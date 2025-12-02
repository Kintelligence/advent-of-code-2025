use shared::parse::Parsable;
use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> Vec<i32> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let mut bytes = line.bytes();

        let positive = bytes.next().unwrap() == b'R';
        let number: i32 = bytes.next_number().unwrap();
        instructions.push(if positive { number } else { -number });
    }

    instructions
}

fn solve_1(instructions: &Vec<i32>) -> usize {
    let mut i = 50;
    let mut solution = 0;

    for instruction in instructions {
        i += instruction;
        i %= 100;
        if i == 0 {
            solution += 1;
        }
    }

    return solution;
}

pub fn part_1(_input: &str) -> Solution {
    let instructions = parse(_input);
    solve_1(&instructions).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 3)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(1023)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

fn solve_2(instructions: &Vec<i32>) -> i32 {
    let mut i = 50;
    let mut solution = 0;

    for instruction in instructions {
        let start_0 = i == 0;
        i += instruction;

        if i == 0 {
            solution += 1;
        } else if i < 0 && !start_0 {
            solution += 1;
        }

        solution += i.abs() / 100;
        i = i.rem_euclid(100);
    }

    return solution;
}

pub fn part_2(_input: &str) -> Solution {
    let instructions = parse(_input);
    solve_2(&instructions).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 6)]
    #[test_case(include_str!("_manual.txt"), 11)]
    fn example_input(input: &str, expected: i32) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(5899)]
    fn real_input(expected: i32) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
