use fxhash::FxHashSet;
use shared::{grid::Grid, points::point::Point, *};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let grid = parse(_input);
    grid.points()
        .filter(|&point| grid[point] && grid.adjacent_eight(point).filter(|&a| grid[a]).count() < 4)
        .count()
        .into()
}

fn parse(input: &str) -> Grid<bool> {
    let mut vec = Vec::new();
    let mut height = 0;

    for line in input.lines() {
        for byte in line.bytes() {
            vec.push(byte == b'@');
        }
        height += 1;
    }

    Grid::from(vec, height)
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 13)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(1604)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let mut grid = parse(_input);
    let mut removed = Grid::filled(false, grid.height, grid.width);
    let mut count: usize = 0;
    let mut relevant: FxHashSet<Point> = grid.points().collect();

    loop {
        if relevant.is_empty() {
            break;
        }

        let to_be_removed: Vec<&Point> = relevant
            .iter()
            .filter(|&&point| {
                grid[point] && grid.adjacent_eight(point).filter(|&a| grid[a]).count() < 4
            })
            .collect();

        for &&point in to_be_removed.iter() {
            grid[point] = false;
            removed[point] = true;
            count += 1;
        }

        relevant = to_be_removed
            .iter()
            .flat_map(|&&n| grid.adjacent_eight(n))
            .filter(|&n| !removed[n])
            .collect();
    }

    count.into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 43)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(9397)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
