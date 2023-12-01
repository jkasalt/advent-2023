use std::time::Instant;

fn p1(input: &str) -> u32 {
    input.lines().map(|l| {
        let first = l.chars().find(|c| c.is_numeric()).and_then(|c| c.to_digit(10)).unwrap();
        let last = l.chars().rfind(|c| c.is_numeric()).and_then(|c| c.to_digit(10)).unwrap();
        first * 10 + last
    }).sum()
}

fn p2(input: &str) -> u32 {
    let replaced = input.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    p1(&replaced)
}

fn main() {
    let input = include_str!("../../input/1.txt");
    let start1 = Instant::now();
    let silver = p1(input);
    let end1 = Instant::now();
    println!("silver: {silver}");
    println!("took: {:?}", end1.duration_since(start1));
    let start2 = Instant::now();
    let gold = p2(input);
    let end2 = Instant::now();
    println!("gold: {gold}");
    println!("took: {:?}", end2.duration_since(start2));
}

mod test_day1 {
    use super::*;

    #[test]
    fn sample1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(p1(input), 142);
    }
    #[test]
    fn sample2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(p2(input), 281);
    }
}
