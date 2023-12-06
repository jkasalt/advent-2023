use core::fmt;
use std::fs;
use std::ops::Range;
use std::time::Instant;

struct Race {
    lasting: i64,
    record: i64,
}

fn parse(input: &str) -> Vec<Race> {
    let times = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|word| word.parse().ok());
    let records = input
        .lines()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|word| word.parse().ok());
    times
        .zip(records)
        .map(|(lasting, record)| Race { lasting, record })
        .collect()
}

fn p1(races: &[Race]) -> usize {
    // races
    //     .iter()
    //     .map(|race| {
    //         let disc = f32::sqrt((race.lasting.pow(2) - 4 * race.record) as f32) / 2.0;
    //         let base = race.lasting as f32 / 2.0;
    //
    //     })
    //     .inspect(|x| println!("{x}"))
    //     .fold(1, |acc, x| acc * x)
    races
        .iter()
        .map(|race| {
            (0..race.lasting)
                .filter(|n| -n.pow(2) + n * race.lasting - race.record > 0)
                .count()
        })
        .fold(1, |acc, x| acc * x)
}

fn parse2(input: &str) -> Race {
    let lasting: String = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let lasting: i64 = lasting.parse().unwrap();

    let record: String = input
        .lines()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let record: i64 = record.parse().unwrap();

    Race { lasting, record }
}

fn p2(race: &Race) -> usize {
    (0..race.lasting)
        .filter(|n| -n.pow(2) + n * race.lasting - race.record > 0)
        .count()
}

fn main() {
    let input = fs::read_to_string("input/6.txt").expect("Input fild should be there");
    let start0 = Instant::now();
    let races = parse(&input);
    let end0 = Instant::now();
    println!("input parsed in {:?}", end0.duration_since(start0));

    let start1 = Instant::now();
    let silver = p1(&races);
    let end1 = Instant::now();
    println!("silver: {silver}");
    println!("took: {:?}", end1.duration_since(start1));

    let race = parse2(&input);
    let start2 = Instant::now();
    let gold = p2(&race);
    let end2 = Instant::now();
    println!("gold: {gold}");
    println!("took: {:?}", end2.duration_since(start2));
}

#[cfg(test)]
mod day6 {
    use super::*;

    const SAMPLE: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn sample1() {
        let races = parse(SAMPLE);
        assert_eq!(p1(&races), 288);
    }

    #[test]
    fn sample2() {
        let race = parse2(SAMPLE);
        assert_eq!(p2(&race), 71503);
    }
}

