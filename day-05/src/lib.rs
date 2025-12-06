#![feature(iter_advance_by)]
use std::ops::Range;

use shared::{parse::Parsable, *};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

fn parse(input: &str) -> (Vec<Range<usize>>, Vec<usize>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut bytes = line.bytes();
        let mut range = Range {
            start: bytes.next_number().unwrap(),
            end: bytes.next_number().unwrap(),
        };
        range.end += 1;
        ranges.push(range);
    }

    while let Some(line) = lines.next() {
        ids.push(line.bytes().next_number().unwrap());
    }

    (ranges, ids)
}

fn consolidate(ranges: &mut Vec<Range<usize>>) -> Vec<Range<usize>> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
    let mut consolidated: Vec<Range<usize>> = Vec::new();

    for next in ranges {
        let mut added = false;
        for range in consolidated.iter_mut() {
            let start_inside = range.contains(&next.start);
            let end_inside = range.contains(&next.end);

            if start_inside && end_inside {
                added = true;
                break;
            } else if start_inside {
                range.end = next.end;
                added = true;
                break;
            }
        }

        if !added {
            consolidated.push(next.clone());
        }
    }

    consolidated
}

pub fn part_1(_input: &str) -> Solution {
    let (mut ranges, ids) = parse(_input);
    let consolidated = consolidate(&mut ranges);
    let outer_range = Range {
        start: consolidated.first().unwrap().start,
        end: consolidated.last().unwrap().end,
    };

    let mut fresh_count: usize = 0;
    for id in ids {
        if outer_range.contains(&id) {
            for range in consolidated.iter() {
                if range.contains(&id) {
                    fresh_count += 1;
                    break;
                }
            }
        }
    }
    fresh_count.into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 3)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(601)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (mut ranges, _ids) = parse(_input);
    let consolidated = consolidate(&mut ranges);
    consolidated
        .iter()
        .map(|range| range.end - range.start)
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 14)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(367899984917516)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
