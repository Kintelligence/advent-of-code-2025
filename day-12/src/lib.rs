#![feature(iter_advance_by)]
use shared::{parse::Parsable, *};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (shapes, problems) = parse(_input);

    problems
        .iter()
        .map(|problem| {
            let area = problem.height as u64 * problem.width as u64;
            let shape_area = problem
                .shape_counts
                .iter()
                .enumerate()
                .fold(0, |acc, (i, count)| {
                    acc + (*count as u64) * (shapes[i].area as u64)
                });
            return if shape_area > area { 0 } else { 1 };
        })
        .sum::<usize>()
        .into()
}

#[derive(Clone, Copy, Debug)]
struct Shape {
    area: u8,
    mask: [u8; 3],
}

#[derive(Clone, Copy, Debug)]
struct Problem {
    height: u8,
    width: u8,
    shape_counts: [u8; 6],
}

fn parse(input: &str) -> ([Shape; 6], Vec<Problem>) {
    let mut shapes = [Shape {
        area: 0,
        mask: [0; 3],
    }; 6];
    let mut problems = Vec::new();
    let mut lines = input.lines();

    for i in 0..6 {
        lines.next();

        let mut area = 0;
        for y in 0..3 {
            if let Some(line) = lines.next() {
                let mut bytes = line.bytes();
                for x in 0..3 {
                    if let Some(byte) = bytes.next()
                        && byte == b'#'
                    {
                        shapes[i].mask[y] |= 0b1 << x;
                        area += 1;
                    }
                }
            }
        }

        shapes[i].area = area;
        lines.next();
    }

    for line in lines {
        let mut bytes = line.bytes();
        problems.push(Problem {
            height: bytes.next_number().unwrap(),
            width: bytes.next_number().unwrap(),
            shape_counts: [
                bytes.next_number().unwrap(),
                bytes.next_number().unwrap(),
                bytes.next_number().unwrap(),
                bytes.next_number().unwrap(),
                bytes.next_number().unwrap(),
                bytes.next_number().unwrap(),
            ],
        });
    }

    (shapes, problems)
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 0)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(433)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}
