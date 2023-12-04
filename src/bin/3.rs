use advent2023::matrix::Matrix;
use std::time::Instant;
use std::{fmt, fs};

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Num { itself: u32, whole: u32, id: usize },
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
            let n = c.to_digit(10).unwrap();
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
                    .fold(0, |n, (i, c)| n + *c * 10u32.pow(i as u32));
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

fn p1(matrix: &Matrix<Cell>) -> u32 {
    let mut will_sum = Vec::new();
    let mut has_symbol = false;
    for ((x, y), cell) in matrix.iter_pos() {
        if let Cell::Num { whole, id, .. } = cell {
            has_symbol |= matrix
                .neighbor_indices(x, y)
                .iter()
                .any(|pos| matches!(matrix[*pos], Cell::Symbol(_)));
            if has_symbol {
                will_sum.push((id, whole));
            }
        } else {
            has_symbol = false;
        }
    }
    will_sum.sort_unstable();
    will_sum.dedup_by_key(|(id, _)| *id);
    will_sum.into_iter().map(|(_, n)| n).sum()
}

fn p2(matrix: &Matrix<Cell>) -> u32 {
    matrix
        .iter_pos()
        .filter(|(_, cell)| **cell == Cell::Symbol('*'))
        .filter_map(|((x, y), _)| {
            let mut found: Vec<_> = matrix
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
            found.sort_unstable();
            found.dedup_by_key(|(id, _)| *id);
            if found.len() == 2 {
                return Some(found.iter().fold(1, |x, (_, y)| x * y));
            }
            None
        })
        .sum()
}

fn main() {
    let input_path = "input/3.txt";
    let input =
        fs::read_to_string(input_path).expect(&format!("input file should be at {input_path}"));
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

    const SAMPLE: &'static str = "467..114..
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
    fn sample1() {
        let input = parse(SAMPLE);
        assert_eq!(p1(&input), 4361);
    }
    #[test]
    fn sample2() {
        let input = parse(SAMPLE);
        assert_eq!(p2(&input), 467835);
    }
}

