#![feature(iter_advance_by)]
use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use fxhash::FxHashMap;
use shared::{math::Compositions, parse::Parsable, solve::State, *};

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| {
            let (goal, buttons, _) = parse_line(line);
            solve_line_1(goal, &buttons)
        })
        .sum::<u128>()
        .into()
}

fn solve_line_1(goal: u16, buttons: &Vec<u16>) -> u128 {
    let mut heap: VecDeque<State<u16>> = VecDeque::new();
    let mut costs: Vec<u128> = vec![u128::MAX; u16::MAX as usize];
    heap.push_back(State { cost: 0, state: 0 });

    while let Some(current) = heap.pop_front() {
        if current.state == goal {
            return current.cost;
        }

        if costs[current.state as usize] > current.cost {
            costs[current.state as usize] = current.cost;
            for button in buttons {
                heap.push_back(State {
                    state: current.state ^ button,
                    cost: current.cost + 1,
                });
            }
        }
    }

    0
}

fn parse_line(line: &str) -> (u16, Vec<u16>, u128) {
    let mut bytes = line.bytes();
    let mut buttons = Vec::new();

    let mut goal_1: u16 = 0;
    let mut i = 0;
    while let Some(next) = bytes.next() {
        match next {
            b']' => break,
            b'#' => {
                goal_1 |= 0b1 << i;
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

    let mut goal_2: u128 = 0;
    let mut i = 0;
    while let Some(next) = Parsable::<u16>::next_number(&mut bytes) {
        goal_2 |= (next as u128) << (i * 10);
        i += 1;
    }

    (goal_1, buttons, goal_2)
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 7)]
    fn example_input(input: &str, expected: u128) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(520)]
    fn real_input(expected: u128) {
        assert_eq!(part_1(_INPUT), expected.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    Solution::None
    /*
    let mut i = 0;
    _input
        .lines()
        .map(|line| {
            let (_, buttons, goal) = parse_line(line);
            println!("solving {i}: {goal}");
            i += 1;
            let sol = solve_line_2(goal, &buttons, &space_out_buttons(&buttons));
            println!("{sol}");
            sol
        })
        .sum::<usize>()
        .into() */
}

fn space_out_buttons(buttons: &Vec<u16>) -> Vec<u128> {
    buttons
        .iter()
        .map(|button| {
            let mut spaced_button: u128 = 0;
            for i in 0..12 {
                if button & 0b1 << i > 0 {
                    spaced_button |= 0b1 << i * 10
                }
            }
            spaced_button
        })
        .collect()
}

fn solve_line_2(goal: u128, buttons: &Vec<u16>, spaced_buttons: &Vec<u128>) -> usize {
    let mut lamps = 0;

    for i in 0..12 {
        if (goal & (0b1111111111 << (i * 10))) > 0 {
            lamps += 1;
        }
    }

    let mut lamp_to_button: Vec<HashSet<usize>> = vec![HashSet::new(); lamps];
    let mut button_to_lamp: Vec<Vec<usize>> = vec![Vec::new(); buttons.len()];

    for (button_id, button) in buttons.iter().enumerate() {
        for i in 0..lamps {
            if button & 0b1 << i > 0 {
                lamp_to_button[i].insert(button_id);
                button_to_lamp[button_id].push(i);
            }
        }
    }
    todo!(); /*
    return brute(
    goal,
    spaced_buttons,
    &(0..buttons.len()).collect(),
    &lamp_to_button,
    &button_to_lamp,
    lamps,
    0,
    &mut FxHashMap::default(),
    ); */
}

fn brute(
    state: u128,
    remaining: &Vec<u16>,
    buttons: &Vec<u16>,
    options: u16,
    lamp_to_button: &Vec<u16>,
    lamps: usize,
    clicks: usize,
    cache: &mut FxHashMap<u128, usize>,
) -> usize {
    todo!();
    /*
    if let Some(best) = cache.get(&state) {
        return *best;
    }

    if state == 0 {
        cache.insert(state, clicks);
        return clicks;
    }
    let lamp_id = find_least_active_lamp(lamps, options, buttons);
    let current_options: HashSet<usize> = options
        .intersection(&lamp_to_button[lamp_id])
        .map(|c| *c)
        .collect();

    let mut maxes = vec![0; lamps];

    for i in 0..lamps {
        maxes[i] = ((remaining >> (i * 10)) & 0b1111111111) as usize;
    }

    let next_options: HashSet<usize> = options.difference(&current_options).map(|c| *c).collect();

    let mut min_clicks = usize::MAX;
    let increments = maxes[lamp_id];

    for combination in Compositions::new(increments, current_options.len()) {
        if let Some(next_remainder) = test_combination(
            remaining,
            buttons,
            combination,
            &maxes,
            &current_options,
            button_to_lamp,
            lamps,
        ) {
            min_clicks = min_clicks.min(brute(
                next_remainder,
                buttons,
                &next_options,
                lamp_to_button,
                button_to_lamp,
                lamps,
                clicks + increments,
                cache,
            ));
        }
    }

    cache.insert(state, min_clicks);
    return min_clicks; */
}

fn test_combination(
    remaining: u128,
    buttons: &Vec<u128>,
    combination: Vec<usize>,
    maxes: &Vec<usize>,
    options: &HashSet<usize>,
    button_to_lamp: &Vec<Vec<usize>>,
    lamps: usize,
) -> Option<u128> {
    let mut incs = vec![0; lamps];
    let mut rem = remaining;
    let mut c = 0;
    for i in options.iter() {
        let current_click = combination[c];
        if current_click > 0 {
            for l in button_to_lamp[*i].iter() {
                if c + incs[*l] > maxes[*l] {
                    return None;
                }
                incs[*l] += c;
            }
        }
        rem -= current_click as u128 * buttons[*i];
        c += 1;
    }

    Some(rem)
}

fn find_least_active_lamp(lamps: usize, options: &HashSet<usize>, buttons: &Vec<u128>) -> usize {
    let mut lamp_button_count = vec![0; lamps];
    for option in options {
        for i in 0..lamps {
            if (buttons[*option] & (0b1 << (i * 10))) > 0 {
                lamp_button_count[i] += 1;
            }
        }
    }

    let mut min = usize::MAX;
    let mut id = 0;
    for (i, &count) in lamp_button_count.iter().enumerate() {
        if count > 0 && count < min {
            min = count;
            id = i;
        }
    }
    id
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 33)]
    #[test_case(include_str!("_short.txt"), 287)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(0)]
    fn real_input(expected: usize) {
        assert_eq!(part_2(_INPUT), expected.into());
    }

    #[test_case(200, 3)]
    #[test_case(40, 5)]
    #[test_case(1, 1)]
    fn combin(a: usize, b: usize) {
        let vec: Vec<Vec<usize>> = Compositions::new(a, b).collect();
        dbg!(vec.len());
    }
}
