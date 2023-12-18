#![feature(iter_map_windows)]

use advent2023::matrix::Matrix;
use core::fmt;
use std::time::Instant;
use std::{fs, vec};

#[derive(Debug)]
enum Direction {
    West,
    South,
    East,
    North,
}

impl Direction {
    fn from_delta((dx, dy): (i64, i64)) -> Option<Self> {
        match (dx, dy) {
            (1.., 0) => Some(Direction::East),
            (..=-1, 0) => Some(Direction::West),
            (0, 1..) => Some(Direction::North),
            (0, ..=-1) => Some(Direction::South),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Cell {
    Ground,
    Vert,
    Horz,
    NToE,
    NToW,
    SToW,
    SToE,
    Starting,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_write = match self {
            Cell::Ground => '.',
            Cell::Vert => '│',
            Cell::Horz => '─',
            Cell::NToE => '└',
            Cell::NToW => '┘',
            Cell::SToW => '┐',
            Cell::SToE => '┌',
            Cell::Starting => 'S',
        };
        write!(f, "{to_write}")
    }
}

fn parse(input: &str) -> Matrix<Cell> {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let items = input
        .chars()
        .filter(|c| c.is_ascii_graphic())
        .map(|c| match c {
            '.' => Cell::Ground,
            '|' => Cell::Vert,
            '-' => Cell::Horz,
            'L' => Cell::NToE,
            'J' => Cell::NToW,
            '7' => Cell::SToW,
            'F' => Cell::SToE,
            'S' => Cell::Starting,
            x => panic!("Unexpected char in input: {x}"),
        });
    Matrix::new(items, width, height)
}

fn find_loop(matrix: &Matrix<Cell>) -> Option<Vec<(usize, usize)>> {
    // Find the starting pos
    let spos = matrix.index_of(|c| *c == Cell::Starting).unwrap();

    // Identify which way is the loop
    matrix
        .rook_neighbor_indices(spos.0, spos.1)
        .find_map(|pos| {
            let mut last_visited = (spos.0 as i64, spos.1 as i64);
            let mut current = (pos.0 as i64, pos.1 as i64);
            let mut visited = vec![spos];

            loop {
                use Cell as C;
                use Direction as D;
                // Follow the path
                visited.push((current.0 as usize, current.1 as usize));
                let last_step =
                    Direction::from_delta((current.0 - last_visited.0, last_visited.1 - current.1))
                        .unwrap();
                last_visited = current;
                let cur_cell = &matrix[(current.0 as usize, current.1 as usize)];
                let delta = match (last_step, cur_cell) {
                    (_, C::Ground) => return None, // invalid path
                    (D::North, C::Vert) => (0, -1),
                    (D::South, C::Vert) => (0, 1),
                    (D::East, C::Horz) => (1, 0),
                    (D::West, C::Horz) => (-1, 0),
                    (D::South, C::NToE) => (1, 0),
                    (D::West, C::NToE) => (0, -1),
                    (D::South, C::NToW) => (-1, 0),
                    (D::East, C::NToW) => (0, -1),
                    (D::North, C::SToW) => (-1, 0),
                    (D::East, C::SToW) => (0, 1),
                    (D::North, C::SToE) => (1, 0),
                    (D::West, C::SToE) => (0, 1),
                    (_, C::Starting) => return Some(visited),
                    _ => return None, // invalid path
                };
                current = (current.0 + delta.0, current.1 + delta.1);
            }
        })
}

fn p1(matrix: &Matrix<Cell>) -> u64 {
    let loop_len = find_loop(matrix).unwrap().len();
    loop_len as u64 / 2
}

fn p2(matrix: &Matrix<Cell>) -> u64 {
    let pipe_loop = find_loop(matrix).unwrap();
    let clean_items = (0..matrix.len())
        .map(|i| (i % matrix.width(), i / matrix.width()))
        .map(|pos| {
            if pipe_loop.contains(&pos) {
                matrix[pos].clone()
            } else {
                Cell::Ground
            }
        });
    let mut clean_matrix = Matrix::new(clean_items, matrix.width(), matrix.height());
    let Some(spos) = clean_matrix.index_of(|cell| *cell == Cell::Starting) else {
        return 0;
    };

    println!("{spos:?}");

    let offsets = [
        (-1, 0), // West
        (1, 0),  // East
        (0, -1), // North
        (0, 1),  // South
    ];

    let connects_well_to_start = [
        clean_matrix
            .get(
                spos.0 as isize + offsets[0].0,
                spos.1 as isize + offsets[0].1,
            )
            .is_some_and(|cell| matches!(cell, Cell::NToE | Cell::SToE | Cell::Horz)),
        clean_matrix
            .get(
                spos.0 as isize + offsets[1].0,
                spos.1 as isize + offsets[1].1,
            )
            .is_some_and(|cell| matches!(cell, Cell::NToW | Cell::SToW | Cell::Horz)),
        clean_matrix
            .get(
                spos.0 as isize + offsets[2].0,
                spos.1 as isize + offsets[2].1,
            )
            .is_some_and(|cell| matches!(cell, Cell::SToW | Cell::SToE | Cell::Vert)),
        clean_matrix
            .get(
                spos.0 as isize + offsets[3].0,
                spos.1 as isize + offsets[3].1,
            )
            .is_some_and(|cell| matches!(cell, Cell::NToW | Cell::NToE | Cell::Vert)),
    ];

    let real_spos = match connects_well_to_start {
        [true, true, false, false] => Cell::Horz,
        [true, false, true, false] => Cell::NToW,
        [true, false, false, true] => Cell::SToW,
        [false, true, true, false] => Cell::NToE,
        [false, true, false, true] => Cell::SToE,
        [false, false, true, true] => Cell::Vert,
        x => panic!("Unexpected connections to start {x:?}"),
    };

    println!("{clean_matrix:?}");

    clean_matrix[spos] = real_spos;

    let mut inside = false;
    let mut entered_border_with = None;
    let mut count = 0;

    for cell in clean_matrix.vec.iter() {
        if inside && *cell == Cell::Ground && entered_border_with.is_none() {
            count += 1;
        }
        if matches!(cell, Cell::Vert) {
            inside = !inside;
        }
        if matches!(cell, Cell::NToE | Cell::NToW | Cell::SToW | Cell::SToE) {
            if let Some(entry_cell) = entered_border_with {
                if matches!(
                    (entry_cell, cell),
                    (Cell::NToE, Cell::SToW) | (Cell::SToE, Cell::NToW)
                ) {
                    inside = !inside;
                }
                entered_border_with = None;
            } else {
                entered_border_with = Some(cell.clone());
            }
        }
    }
    count
}

fn main() {
    let input_path = "input/10.txt";
    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|_| panic!("input file should be at {input_path}"));
    let start0 = Instant::now();
    let matrix = parse(&input);
    let end0 = Instant::now();
    println!("input parsed in {:?}", end0.duration_since(start0));

    let start1 = Instant::now();
    let silver = p1(&matrix);
    let end1 = Instant::now();
    println!("silver: {silver}");
    println!("took: {:?}", end1.duration_since(start1));

    let start2 = Instant::now();
    let gold = p2(&matrix);
    let end2 = Instant::now();
    println!("gold: {gold}");
    println!("took: {:?}", end2.duration_since(start2));
}

#[cfg(test)]
mod day10 {
    use super::*;

    const SAMPLE1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const SAMPLE2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn p1s1() {
        let matrix = parse(SAMPLE1);
        let result = p1(&matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn p1s2() {
        let matrix = parse(SAMPLE2);
        let result = p1(&matrix);
        assert_eq!(result, 8);
    }

    const SAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const SAMPLE4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const SAMPLE5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn p2s3() {
        let matrix = parse(SAMPLE3);
        let result = p2(&matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn p2s4() {
        let matrix = parse(SAMPLE4);
        let result = p2(&matrix);
        assert_eq!(result, 8);
    }

    #[test]
    fn p2s5() {
        let matrix = parse(SAMPLE5);
        let result = p2(&matrix);
        assert_eq!(result, 10);
    }
}
