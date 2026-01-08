#![feature(iter_advance_by)]
use itertools::Itertools;
use shared::{
    parse::Parsable,
    points::{iline::ILine, ipoint::IPoint},
    *,
};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    parse(_input)
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .into()
}

fn parse(input: &str) -> Vec<IPoint> {
    input
        .lines()
        .map(|line| {
            let mut bytes = line.bytes();
            return IPoint {
                x: bytes.next_number().unwrap(),
                y: bytes.next_number().unwrap(),
            };
        })
        .collect()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 50)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(4749672288)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

struct Rectangle {
    a: IPoint,
    b: IPoint,
    area: usize,
}

struct TileLine<'a> {
    line: &'a ILine,
    height: isize,
    under_is_inside: bool,
}

fn build_tile_lines(lines: &'_ Vec<ILine>, right_oriented: bool) -> Vec<TileLine<'_>> {
    let mut tile_lines: Vec<TileLine> = lines
        .iter()
        .filter_map(|line| match line.from.x.cmp(&line.to.x) {
            std::cmp::Ordering::Less => Some(TileLine {
                line: line,
                height: line.from.y,
                under_is_inside: right_oriented,
            }),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(TileLine {
                line: line,
                height: line.from.y,
                under_is_inside: !right_oriented,
            }),
        })
        .collect();

    tile_lines.sort_by(|a, b| b.height.cmp(&a.height));
    tile_lines
}

pub fn part_2(_input: &str) -> Solution {
    let points = parse(_input);
    
    let mut lines: Vec<ILine> = points.windows(2).map(|p| ILine::new(p[0], p[1])).collect();
    lines.push(ILine::new(
        *points.last().unwrap(),
        *points.first().unwrap(),
    ));

    let right_oriented = lines
        .windows(2)
        .map(|slice| slice[0].orient(&slice[1].to).signum())
        .sum::<isize>()
        > 0;

    let tile_lines = build_tile_lines(&lines, right_oriented);

    let mut rectangles: Vec<Rectangle> = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Rectangle {
            a: *a,
            b: *b,
            area: (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1),
        })
        .collect();

    rectangles.sort_by(|a, b| b.area.cmp(&a.area));

    for rectangle in rectangles {
        let c = IPoint {
            x: rectangle.a.x,
            y: rectangle.b.y,
        };

        let d = IPoint {
            x: rectangle.b.x,
            y: rectangle.a.y,
        };

        let center = IPoint {
            x: rectangle.a.x.min(rectangle.b.x)
                + (rectangle.a.x.abs_diff(rectangle.b.x) as isize / 2),
            y: rectangle.a.y.min(rectangle.b.y)
                + (rectangle.a.y.abs_diff(rectangle.b.y) as isize / 2),
        };

        if !point_is_inside(&center, &tile_lines)
            || !point_is_inside(&c, &tile_lines)
            || !point_is_inside(&d, &tile_lines)
            || inner_rectangle_is_pierced(&rectangle, &lines)
        {
            continue;
        }

        return rectangle.area.into();
    }

    Solution::None
}

fn inner_rectangle_is_pierced(rectangle: &Rectangle, lines: &Vec<ILine>) -> bool {
    let min_x = rectangle.a.x.min(rectangle.b.x) + 1;
    let min_y = rectangle.a.y.min(rectangle.b.y) + 1;
    let max_x = rectangle.a.x.max(rectangle.b.x) - 1;
    let max_y = rectangle.a.y.max(rectangle.b.y) - 1;

    let inner_corners = vec![
        IPoint::new(min_x, min_y),
        IPoint::new(min_x, max_y),
        IPoint::new(max_x, max_y),
        IPoint::new(max_x, min_y),
        IPoint::new(min_x, min_y),
    ];
    for inner_line in inner_corners
        .windows(2)
        .map(|slice| ILine::new(slice[0], slice[1]))
    {
        for line in lines {
            if line.crosses_straight(&inner_line) {
                return true;
            }
        }
    }

    false
}

fn point_is_inside(point: &IPoint, tile_lines: &Vec<TileLine>) -> bool {
    let trace = ILine::new(IPoint { x: point.x, y: -1 }, *point);
    for tile_line in tile_lines {
        if point.y >= tile_line.height && tile_line.line.crosses_straight(&trace) {
            if point.y == tile_line.height {
                return true;
            }
            return tile_line.under_is_inside;
        }
    }
    false
}

pub fn validate(input: &str) {
    let points = parse(input);
    for (a, b) in points.iter().tuple_combinations() {
        if a.x.abs_diff(b.x) == 1 {
            println!("Points are too close {} {}", a, b)
        }
        if a.y.abs_diff(b.y) == 1 {
            println!("Points are too close {} {}", a, b)
        }
    }

    let mut lines: Vec<ILine> = points.windows(2).map(|p| ILine::new(p[0], p[1])).collect();
    lines.push(ILine::new(
        *points.last().unwrap(),
        *points.first().unwrap(),
    ));

    let mut left = 0;
    let mut right = 0;

    for slice in lines.windows(2) {
        let ori = slice[0].orient(&slice[1].to);
        if ori == 0 {
            println!("bad line {} {}", slice[0], slice[1]);
        }
        if ori > 0 {
            right += 1;
        } else {
            left += 1;
        }
    }

    println!("left: {} right: {}", left, right);

    println!("finished validating");
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 24)]
    #[test_case(include_str!("_amogus.txt"), 54)]
    #[test_case(include_str!("_manual.txt"), 24)]
    #[test_case(include_str!("_alex.txt"), 39)]
    #[test_case(include_str!("_reddit.txt"), 30)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(1479665889)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }

    #[test_case(include_str!("_reddit.txt"))]
    fn validate_example(input: &str) {
        validate(input);
    }

    #[test]
    fn validate_input() {
        validate(_INPUT);
    }
}
