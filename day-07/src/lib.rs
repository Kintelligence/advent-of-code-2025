use shared::Solution;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (lines, start, width) = parse(_input);

    let mut beams: Vec<bool> = vec![false; width];
    let mut count: usize = 0;

    beams[start] = true;

    for line in lines {
        for (x, b) in line.bytes().enumerate() {
            if b == b'^' && beams[x] {
                beams[x - 1] = true;
                beams[x] = false;
                beams[x + 1] = true;
                count += 1;
            }
        }
    }

    count.into()
}

fn parse(input: &str) -> (impl Iterator<Item = &str>, usize, usize) {
    let mut lines = input.lines().step_by(2);
    let mut start = 0;
    let mut width = 0;
    if let Some(line) = lines.next() {
        for byte in line.bytes() {
            if byte == b'S' {
                start = width;
            }
            width += 1;
        }
    }

    (lines, start, width)
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 21)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(1587)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (lines, start, width) = parse(_input);
    let mut beams: Vec<usize> = vec![0; width];
    beams[start] = 1;

    for line in lines {
        for (x, b) in line.bytes().enumerate() {
            if b == b'^' {
                beams[x - 1] += beams[x];
                beams[x + 1] += beams[x];
                beams[x] = 0;
            }
        }
    }

    beams.iter().sum::<usize>().into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 40)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(5748679033029)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
