#![feature(iter_advance_by)]
use shared::{parse::Parsable, points::point3d::Point3d, *};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    solve_1(_input, 1_000).into()
}

fn solve_1(input: &str, connection_count: usize) -> usize {
    let points = parse(input);
    let mut connections = build_connections(&points);

    let mut sets: Vec<Vec<usize>> = Vec::new();
    let mut point_to_set: Vec<Option<usize>> = vec![None; points.len()];
    let mut next_set = 0;

    for _ in 0..connection_count {
        if let Some(connection) = connections.pop() {
            let opt_a = point_to_set[connection.a];
            let opt_b = point_to_set[connection.b];

            match (opt_a, opt_b) {
                (None, None) => {
                    point_to_set[connection.a] = Some(next_set);
                    point_to_set[connection.b] = Some(next_set);
                    sets.push(vec![connection.a, connection.b]);
                    next_set += 1;
                }
                (None, Some(set)) => {
                    point_to_set[connection.a] = Some(set);
                    sets[set].push(connection.a);
                }
                (Some(set), None) => {
                    point_to_set[connection.b] = Some(set);
                    sets[set].push(connection.b);
                }
                (Some(set_a), Some(set_b)) => {
                    if set_a != set_b {
                        while let Some(p) = sets[set_b].pop() {
                            point_to_set[p] = Some(set_a);
                            sets[set_a].push(p);
                        }
                    }
                }
            }
        }
    }

    let mut t1 = 0;
    let mut t2 = 0;
    let mut t3 = 0;

    for set in sets {
        let count = set.len();

        if count > t1 {
            t3 = t2;
            t2 = t1;
            t1 = count;
        } else if count > t2 {
            t3 = t2;
            t2 = count;
        } else if count > t3 {
            t3 = count;
        }
    }

    t1 * t2 * t3
}

fn parse(input: &str) -> Vec<Point3d> {
    input
        .lines()
        .map(|line| {
            let mut bytes = line.bytes();
            Point3d {
                x: bytes.next_number().unwrap(),
                y: bytes.next_number().unwrap(),
                z: bytes.next_number().unwrap(),
            }
        })
        .collect()
}

struct Connection {
    dist: f64,
    a: usize,
    b: usize,
}

fn build_connections(points: &Vec<Point3d>) -> Vec<Connection> {
    let mut connections = Vec::new();
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            connections.push(Connection {
                dist: (points[i] - points[j]).non_sqrt_magnitude(),
                a: i,
                b: j,
            });
        }
    }

    connections.sort_by(|a, b| {
        b.dist
            .partial_cmp(&a.dist)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    connections
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 40)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(solve_1(input, 10), expected.into());
    }

    #[test_case(42840)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let points = parse(_input);
    let mut connections = build_connections(&points);

    let mut sets: Vec<Vec<usize>> = Vec::new();
    let mut point_to_set: Vec<Option<usize>> = vec![None; points.len()];
    let mut next_set = 0;
    let mut count = points.len();

    loop {
        if let Some(connection) = connections.pop() {
            let opt_a = point_to_set[connection.a];
            let opt_b = point_to_set[connection.b];

            match (opt_a, opt_b) {
                (None, None) => {
                    point_to_set[connection.a] = Some(next_set);
                    point_to_set[connection.b] = Some(next_set);
                    sets.push(vec![connection.a, connection.b]);
                    next_set += 1;
                    count -= 1;
                }
                (None, Some(set)) => {
                    point_to_set[connection.a] = Some(set);
                    sets[set].push(connection.a);
                    count -= 1;
                }
                (Some(set), None) => {
                    point_to_set[connection.b] = Some(set);
                    sets[set].push(connection.b);
                    count -= 1;
                }
                (Some(set_a), Some(set_b)) => {
                    if set_a != set_b {
                        while let Some(p) = sets[set_b].pop() {
                            point_to_set[p] = Some(set_a);
                            sets[set_a].push(p);
                        }
                        count -= 1;
                    }
                }
            }

            if count == 1 {
                return (points[connection.a].x as usize * points[connection.b].x as usize).into();
            }
        }
    }
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 25272)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(0)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
