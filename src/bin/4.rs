use std::collections::HashSet;
use std::fs;
use std::time::Instant;

struct Card {
    winning: HashSet<u64>,
    have: HashSet<u64>,
}

impl Card {
    fn points(&self) -> u64 {
        let m = self.matching();
        if m == 0 {
            0
        } else {
            2u64.pow(m as u32 - 1)
        }
    }

    fn matching(&self) -> usize {
        self.have.intersection(&self.winning).count()
    }
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|l| {
            let (_, rest) = l.split_once(':').expect("input should be valid");
            let (winning_part, have_part) = rest.split_once('|').expect("input should be valid");
            let winning = winning_part
                .split_whitespace()
                .map(|w| w.parse().expect("should be a number"))
                .collect();
            let have = have_part
                .split_whitespace()
                .map(|w| w.parse().expect("should be a number"))
                .collect();
            Card { winning, have }
        })
        .collect()
}

fn p1(cards: &[Card]) -> u64 {
    cards.iter().map(|card| card.points()).sum()
}

fn p2(cards: &[Card]) -> usize {
    let mut how_many = vec![1; cards.len()];
    'outer: for (i, card) in cards.iter().enumerate() {
        let wh = card.matching();
        let amount = how_many[i];
        for j in (i + 1)..=(i + wh) {
            if let Some(other) = how_many.get_mut(j) {
                *other += amount;
            } else {
                continue 'outer;
            }
        }
    }
    how_many.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input/4.txt").expect("Input fild should be there");
    let start0 = Instant::now();
    let cards = parse(&input);
    let end0 = Instant::now();
    println!("input parsed in {:?}", end0.duration_since(start0));

    let start1 = Instant::now();
    let silver = p1(&cards);
    let end1 = Instant::now();
    println!("silver: {silver}");
    println!("took: {:?}", end1.duration_since(start1));

    let start2 = Instant::now();
    let gold = p2(&cards);
    let end2 = Instant::now();
    println!("gold: {gold}");
    println!("took: {:?}", end2.duration_since(start2));
}

#[cfg(test)]
mod day4 {
    use super::*;

    const SAMPLE: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn sample1() {
        let cards = parse(SAMPLE);
        assert_eq!(p1(&cards), 13);
    }

    #[test]
    fn sample2() {
        let cards = parse(SAMPLE);
        assert_eq!(p2(&cards), 30);
    }
}

