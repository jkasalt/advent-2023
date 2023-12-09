#![feature(iter_map_windows)]

use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

fn parse(input: &str) -> Vec<VecDeque<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .filter_map(|w| w.parse().ok())
                .collect()
        })
        .collect()
}

fn solve(histories: &[VecDeque<i64>], part2: bool) -> i64 {
    let mut result = 0;
    for history in histories {
        // drive it down
        let mut derivatives = vec![history.clone()];
        loop {
            let new_der: VecDeque<_> = derivatives
                .last()
                .unwrap()
                .iter()
                .map_windows(|[a, b]| *b - *a)
                .collect();
            let finished = new_der.iter().all(|n| *n == 0);
            derivatives.push(new_der);
            if finished {
                break;
            }
        }
        // build it up
        if part2 {
            derivatives.last_mut().unwrap().push_front(0);
            for i in (0..derivatives.len()).rev().skip(1) {
                let base = derivatives[i + 1][0];
                let oldest = derivatives[i][0];
                derivatives[i].push_front(oldest - base);
            }
            result += derivatives[0][0];
        } else {
            derivatives.last_mut().unwrap().push_back(0);
            for i in (0..derivatives.len()).rev().skip(1) {
                let base = *derivatives[i + 1].iter().last().unwrap();
                let latest = *derivatives[i].iter().last().unwrap();
                derivatives[i].push_back(base + latest);
            }
            // Get the result
            result += derivatives[0].iter().last().unwrap();
        }
    }

    result
}

fn main() {
    let input_path = "input/9.txt";
    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|_| panic!("input file should be at {input_path}"));
    let start0 = Instant::now();
    let histories = parse(&input);
    let end0 = Instant::now();
    println!("input parsed in {:?}", end0.duration_since(start0));

    let start1 = Instant::now();
    let silver = solve(&histories, false);
    let end1 = Instant::now();
    println!("silver: {silver}");
    println!("took: {:?}", end1.duration_since(start1));

    let start2 = Instant::now();
    let gold = solve(&histories, true);
    let end2 = Instant::now();
    println!("gold: {gold}");
    println!("took: {:?}", end2.duration_since(start2));
}

#[cfg(test)]
mod day9 {
    use super::*;

    const SAMPLE: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn sample1() {
        let histories = parse(SAMPLE);
        assert_eq!(solve(&histories, false), 114);
    }

    #[test]
    fn sample2() {
        let histories = parse(SAMPLE);
        assert_eq!(solve(&histories, true), 2);
    }
}

