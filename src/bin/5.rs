use core::fmt;
use std::fs;
// use std::ops::Range;
use std::time::Instant;

struct Mapper {
    dst: u64,
    src: u64,
    range: u64,
}

impl Mapper {
    fn contains(&self, num: &u64) -> bool {
        (self.src..(self.src + self.range)).contains(num)
    }

    // fn end(&self) -> u64 {
    //     self.src + self.range
    // }

    fn map(&self, num: u64) -> u64 {
        num - self.src + self.dst
    }
}

impl fmt::Debug for Mapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}..{} -> n - {} + {}",
            self.src,
            self.src + self.range,
            self.src,
            self.dst
        )?;
        Ok(())
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Mapper>>) {
    let seeds: Vec<u64> = input
        .lines()
        .next()
        .and_then(|l| l.split_once(':'))
        .map(|l| {
            l.1.split_ascii_whitespace()
                .map(|w| w.parse().unwrap())
                .collect()
        })
        .expect("input should be valid");
    let mappers = input
        .split("\n\n")
        .skip(1)
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line
                        .split_ascii_whitespace()
                        .map(|word| word.parse().unwrap());
                    Mapper {
                        dst: nums.next().unwrap(),
                        src: nums.next().unwrap(),
                        range: nums.next().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    (seeds, mappers)
}

fn p1(seeds: &[u64], mappers: &[Vec<Mapper>]) -> u64 {
    seeds
        .iter()
        .map(|seed| {
            let mut num = *seed;
            'section: for section in mappers {
                for mapper in section {
                    if mapper.contains(&num) {
                        num = mapper.map(num);
                        continue 'section;
                    }
                }
            }
            num
        })
        .min()
        .unwrap()
}

fn p2(seeds: &[u64], mappers: &[Vec<Mapper>]) -> u64 {
    let seeds: Vec<u64> = seeds.chunks(2).flat_map(|c| c[0]..(c[0] + c[1])).collect();
    p1(&seeds, &mappers)
    // let seed_ranges: Vec<Range<u64>> = seeds.chunks(2).map(|c| c[0]..(c[0] + c[1])).collect();
    // println!("seed_ranges: {seed_ranges:?}");
    // seed_ranges
    //     .into_iter()
    //     .flat_map(|seed_range| {
    //         let mut next_range = vec![Some(seed_range)];
    //         for section in mappers {
    //             println!("---");
    //             let mut mapped: Vec<Range<u64>> = Vec::new();
    //             for maybe_range in &mut next_range {
    //                 let mut should_be_none = false;
    //                 if let Some(range) = maybe_range {
    //                     for mapper in section {
    //                         println!("range is {range:?}, mapper is {mapper:?}...");
    //                         // if the range is fully in
    //                         if mapper.contains(&range.start) && mapper.contains(&range.end) {
    //                             println!("found completely contained!");
    //                             let new_start = mapper.map(range.start);
    //                             let new_end = mapper.map(range.end);
    //                             println!("mapped to {new_start}..{new_end}");
    //                             mapped.push(new_start..new_end);
    //                             should_be_none = true;
    //                         } else if mapper.contains(&range.end) {
    //                             // if the end is in, we have to split this range and map the tail
    //                             println!("found the end is contained");
    //                             let tail = range.end;
    //                             range.end = mapper.src - 1;
    //
    //                             let new_start = mapper.map(mapper.src);
    //                             let new_end = mapper.map(tail);
    //                             println!(
    //                                 "split between {}..{}, and {new_start}..{new_end}",
    //                                 range.start, range.end
    //                             );
    //                             mapped.push(new_start..new_end);
    //                         } else if mapper.contains(&range.start) {
    //                             // if the start is in, we have to split this range and map the head
    //                             println!("found the start is contained");
    //                             let head = range.start;
    //                             range.start = mapper.end();
    //                             let new_start = mapper.map(head);
    //                             let new_end = mapper.map(mapper.end() - 1);
    //                             println!(
    //                                 "split between {}..{}, and {new_start}..{new_end}",
    //                                 range.start, range.end
    //                             );
    //                             mapped.push(new_start..new_end);
    //                         } else {
    //                             println!("not contained");
    //                         }
    //                     }
    //                 }
    //                 if should_be_none {
    //                     *maybe_range = None;
    //                 }
    //             }
    //             next_range.extend(mapped.into_iter().map(|r| Some(r)));
    //         }
    //         dbg!(next_range)
    //     })
    //     .filter_map(|range| range.map(|r| r.start))
    //     .min()
    //     .unwrap()
}

fn main() {
    let input = fs::read_to_string("input/5.txt").expect("Input fild should be there");
    let start0 = Instant::now();
    let (seeds, mappers) = parse(&input);
    let end0 = Instant::now();
    println!("input parsed in {:?}", end0.duration_since(start0));

    let start1 = Instant::now();
    let silver = p1(&seeds, &mappers);
    let end1 = Instant::now();
    println!("silver: {silver}");
    println!("took: {:?}", end1.duration_since(start1));

    let start2 = Instant::now();
    let gold = p2(&seeds, &mappers);
    let end2 = Instant::now();
    println!("gold: {gold}");
    println!("took: {:?}", end2.duration_since(start2));
}

#[cfg(test)]
mod day5 {
    use super::*;

    const SAMPLE: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn sample1() {
        let (seeds, mappers) = parse(SAMPLE);
        assert_eq!(p1(&seeds, &mappers), 35);
    }

    #[test]
    fn sample2() {
        let (seeds, mappers) = parse(SAMPLE);
        assert_eq!(p2(&seeds, &mappers), 46)
    }
}

