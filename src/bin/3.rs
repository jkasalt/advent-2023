use advent2023::matrix::Matrix;
use std::collections::HashMap;
use std::time::Instant;
use std::{env, fmt, fs};

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Num { itself: u64, whole: u64, id: usize },
    Symbol(char),
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Num { itself, .. } => write!(f, "{itself}"),
            Cell::Symbol(x) => write!(f, "{x}"),
        }
    }
}

fn parse(input: &str) -> Matrix<Cell> {
    let width = input
        .lines()
        .next()
        .expect("input should not be empty")
        .chars()
        .count();
    let height = input.lines().count();
    let mut items = Vec::new();
    let mut buf = Vec::new();
    let mut remember = Vec::new();
    for c in input.chars().filter(|c| c.is_ascii_graphic()) {
        if c.is_numeric() {
            let n = c.to_digit(10).unwrap() as u64;
            let cell = Cell::Num {
                itself: n,
                whole: 0,
                id: items.len() - remember.len(),
            };
            items.push(cell);
            remember.push(items.len() - 1);
            buf.push(n);
        } else {
            match c {
                '.' => items.push(Cell::Empty),
                x => items.push(Cell::Symbol(x)),
            }
            if !buf.is_empty() {
                // Reset the state
                let num = buf
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |n, (i, c)| n + *c * 10u64.pow(i as u32));
                buf.clear();
                for i in &remember {
                    match items[*i] {
                        Cell::Num { ref mut whole, .. } => *whole = num,
                        _ => {}
                    }
                }
                remember.clear();
            }
        }
    }
    Matrix::new(items, width, height)
}

fn p1(matrix: &Matrix<Cell>) -> u64 {
    let mut will_sum = HashMap::new();
    let mut has_symbol = false;
    for ((x, y), cell) in matrix.iter_pos() {
        if let Cell::Num { whole, id, .. } = cell {
            has_symbol |= matrix
                .neighbor_indices(x, y)
                .iter()
                .any(|pos| matches!(matrix[*pos], Cell::Symbol(_)));
            if has_symbol {
                will_sum.insert(*id, *whole);
            }
        } else {
            has_symbol = false;
        }
    }
    dbg!(will_sum).values().sum()
}

fn p2(matrix: &Matrix<Cell>) -> u64 {
    matrix
        .iter_pos()
        .filter(|(_, cell)| **cell == Cell::Symbol('*'))
        .filter_map(|((x, y), _)| {
            let found: HashMap<_, _> = matrix
                .neighbor_indices(x, y)
                .iter()
                .filter_map(|pos| {
                    if let Cell::Num { whole, id, .. } = matrix[*pos] {
                        Some((id, whole))
                    } else {
                        None
                    }
                })
                .collect();
            if found.len() == 2 {
                Some(found.values().fold(1, |x, y| x * y))
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = {
        if env::args()
            .find(|s| matches!(s.as_str(), "--bigboy"))
            .is_some()
        {
            fs::read_to_string("bigboy/3.txt").unwrap()
            // Bigboy
            // silver: 258006204
            // gold: 17158526595
        } else {
            fs::read_to_string("input/3.txt").unwrap()
        }
    };
    let start0 = Instant::now();
    let input = parse(&input);
    let end0 = Instant::now();
    println!("input parsed in {:?}", end0.duration_since(start0));

    let start1 = Instant::now();
    let silver = p1(&input);
    let end1 = Instant::now();
    println!("silver: {silver}");
    println!("took: {:?}", end1.duration_since(start1));

    let start2 = Instant::now();
    let gold = p2(&input);
    let end2 = Instant::now();
    println!("gold: {gold}");
    println!("took: {:?}", end2.duration_since(start2));
}

#[cfg(test)]
mod day3 {
    use super::*;

    const SAMPLE1: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn sample1_p1() {
        let input = parse(SAMPLE1);
        assert_eq!(p1(&input), 4361);
    }
    #[test]
    fn sample1_p2() {
        let input = parse(SAMPLE1);
        assert_eq!(p2(&input), 467835);
    }

    const SAMPLE2: &'static str = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";

    #[test]
    fn sample2_p1() {
        let input = parse(SAMPLE2);
        assert_eq!(p1(&input), 925);
    }
    #[test]
    fn sample2_p2() {
        let input = parse(SAMPLE2);
        assert_eq!(p2(&input), 6756);
    }
}

