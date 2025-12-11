#![feature(iter_advance_by)]
use std::iter::from_fn;

use fxhash::FxHashMap;
use shared::{misc::translator::Translator, *};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (graph, mut translator) = parse(_input);
    count(YOU, OUT, &graph, &mut translator).into()
}

fn count(from: u16, to: u16, graph: &Vec<Vec<u16>>, translator: &mut Translator<u16>) -> usize {
    recursive_count(
        translator.translate(from) as u16,
        translator.translate(to) as u16,
        &graph,
        &mut vec![None; graph.len()],
    )
}

fn recursive_count(
    i: u16,
    target: u16,
    graph: &Vec<Vec<u16>>,
    cache: &mut Vec<Option<usize>>,
) -> usize {
    if let Some(result) = cache[i as usize] {
        return result;
    }

    if i == target {
        return 1;
    }

    if i == 0 {
        return 0;
    }

    let result = graph[i as usize]
        .iter()
        .map(|d| recursive_count(*d, target, graph, cache))
        .sum();
    cache[i as usize] = Some(result);
    result
}

const OUT: u16 = 10003;
const YOU: u16 = 16608;
const SVR: u16 = 12731;
const DAC: u16 = 2030;
const FFT: u16 = 3529;

fn parse(input: &str) -> (Vec<Vec<u16>>, Translator<u16>) {
    let mut translator = Translator::new();
    let mut vec = Vec::new();

    translator.translate(OUT);
    vec.push(Vec::new());

    let mut iters: Vec<_> = input.lines().map(|line| line.bytes()).collect();

    for mut iter in iters.iter_mut() {
        let start = next_id(&mut iter).unwrap();
        translator.translate(start);
    }

    for mut iter in iters.iter_mut() {
        vec.push(
            from_fn(|| next_id(&mut iter).and_then(|x| Some(translator.translate(x) as u16)))
                .collect(),
        );
    }

    (vec, translator)
}

fn next_id<T: Iterator<Item = u8>>(input: &mut T) -> Option<u16> {
    let mut id: u16 = 0;
    let mut empty = true;
    while let Some(next) = input.next() {
        if next.is_ascii_lowercase() {
            id *= 26;
            id += (next - b'a') as u16;
            empty = false;
        } else if !empty {
            return Some(id);
        }
    }
    if empty {
        return None;
    }
    Some(id)
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test1.txt"), 5)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(431)]
    fn real_input(expected: usize) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (graph, mut translator) = parse(_input);

    let mut total = count(DAC, FFT, &graph, &mut translator);
    if total == 0 {
        total = count(SVR, FFT, &graph, &mut translator);
        total *= count(FFT, DAC, &graph, &mut translator);
        total *= count(DAC, OUT, &graph, &mut translator);
    } else {
        total *= count(SVR, DAC, &graph, &mut translator);
        total *= count(FFT, OUT, &graph, &mut translator);
    }

    total.into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test2.txt"), 2)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(358458157650450)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }
}
