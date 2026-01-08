#![feature(iter_advance_by)]
use core::num;
use fxhash::FxHashMap;
use itertools::Itertools;
use shared::{parse::Parsable, *};
use std::{
    iter::from_fn,
    ops::{Add, Shl, Shr, Sub},
    u64,
};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| {
            let (goal, buttons) = parse_line_1(line);
            solve_line_1(goal, &buttons)
        })
        .sum::<u32>()
        .into()
}

fn solve_line_1(goal: u16, buttons: &Vec<u16>) -> u32 {
    (0..=buttons.len())
        .flat_map(|k| buttons.iter().combinations(k))
        .filter_map(|combination| {
            let result = combination.iter().map(|&b| *b).fold(0, |acc, x| acc ^ x);
            if result == goal {
                return Some(combination.iter().len() as u32);
            }
            None
        })
        .min()
        .unwrap_or(0)
}

fn parse_line_1(line: &str) -> (u16, Vec<u16>) {
    let mut bytes = line.bytes();
    let mut buttons = Vec::new();

    let mut goal: u16 = 0;
    let mut i = 0;
    while let Some(next) = bytes.next() {
        match next {
            b']' => break,
            b'#' => {
                goal |= 0b1 << i;
                i += 1
            }
            b'.' => i += 1,
            _ => continue,
        }
    }

    let mut current = 0;
    while let Some(next) = bytes.next() {
        match next {
            b'{' => break,
            b')' => {
                buttons.push(current);
                current = 0;
            }
            b => {
                if b.is_ascii_digit() {
                    let x = b - b'0';
                    current |= 0b1 << x;
                }
            }
        }
    }

    (goal, buttons)
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 7)]
    fn example_input(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(520)]
    fn real_input(expected: u32) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

struct Machine {
    buttons: Vec<u128>,
    total: u128,
}

fn parse_line_2(input: &str) -> Machine {
    let mut bytes = input.bytes();
    let mut buttons = Vec::new();

    let mut current: u128 = 0;
    while let Some(next) = bytes.next() {
        match next {
            b'{' => break,
            b')' => {
                buttons.push(current);
                current = 0;
            }
            b => {
                if b.is_ascii_digit() {
                    let x = b - b'0';
                    current |= 0b1 << x * 10;
                }
            }
        }
    }

    Machine {
        buttons: buttons,
        total: from_fn(|| Parsable::<u128>::next_number(&mut bytes))
            .enumerate()
            .fold(0, |acc, (i, x)| acc | (x << i * 10)),
    }
}

pub fn part_2(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| solve_line_2(&parse_line_2(line)))
        .sum::<u64>()
        .into()
}

fn solve_line_2(machine: &Machine) -> u64 {
    solve_rec(
        machine.total,
        &(0..=machine.buttons.len())
            .flat_map(|k| machine.buttons.iter().combinations(k))
            .map(|c| (c.iter().map(|&b| *b).sum::<u128>(), c.iter().len() as u8))
            .collect(),
        &mut FxHashMap::default(),
    )
    .unwrap_or(0)
}

fn solve_rec(
    remaining: u128,
    options: &Vec<(u128, u8)>,
    cache: &mut FxHashMap<u128, Option<u64>>,
) -> Option<u64> {
    if let Some(best) = cache.get(&remaining) {
        return *best;
    }

    if remaining == 0 {
        return Some(0);
    }

    let best = options
        .iter()
        .filter_map(|(sum, cost)| {
            if let Some(result) = checked_sub(remaining, *sum)
                && is_even(result)
                && let Some(res) = solve_rec(result >> 1, options, cache)
            {
                Some(res * 2 + *cost as u64)
            } else {
                None
            }
        })
        .min();

    cache.insert(remaining, best);
    best
}

#[inline]
fn checked_sub(input: u128, other: u128) -> Option<u128> {
    for i in 0..10 {
        let mask: u128 = 0b1111111111 << (i * 10);
        if (input & mask) < (other & mask) {
            return None;
        }
    }
    Some(input - other)
}

#[inline]
fn is_even(input: u128) -> bool {
    for i in 0..10 {
        if (input & (0b1 << (i * 10))) > 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 33)]
    #[test_case(include_str!("_short.txt"), 287)]
    #[test_case(include_str!("_panda.txt"), 19293)]
    fn example_input(input: &str, expected: u64) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(20626)]
    fn real_input(expected: u64) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
