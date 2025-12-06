use shared::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .filter_map(|line| solve(line.as_bytes(), 0, 2))
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
        .filter_map(|line| solve(line.as_bytes(), 0, 12))
        .sum::<usize>()
        .into()
}

fn solve(bytes: &[u8], pos: usize, rem: usize) -> Option<usize> {
    if rem == 0 {
        return None;
    }

    let mut max = 0;
    let mut next = 0;

    for i in pos..bytes.len() - rem + 1 {
        let a = bytes[i];
        if a > max {
            max = a;
            next = i;
        }
    }

    if let Some(s) = solve(bytes, next + 1, rem - 1) {
        Some((max - b'0') as usize * 10_usize.pow(rem as u32 - 1) + s)
    } else {
        Some((max - b'0') as usize)
    }
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
