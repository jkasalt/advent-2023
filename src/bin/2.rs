use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

static RE_RED: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) red").unwrap());
static RE_GREEN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) green").unwrap());
static RE_BLUE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) blue").unwrap());

fn get_num(re: &Regex, s: &str) -> Result<u32> {
    if let Some(cap) = re.captures(s) {
        cap.get(1)
            .context("Regex does not have at least 1 capture group")
            .and_then(|d| d.as_str().parse().context("Failed to parse int"))
    } else {
        Ok(0)
    }
}

#[derive(Default)]
struct Rgb {
    red: u32,
    green: u32,
    blue: u32,
}

impl Rgb {
    fn max(&self, other: &Rgb) -> Rgb {
        Rgb {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: usize,
    shown: Vec<Rgb>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (id_part, shown_part) = s.split_once(": ").context("Failed to split input")?;
        let id = id_part
            .split_ascii_whitespace()
            .nth(1)
            .and_then(|word| word.parse().ok())
            .with_context(|| format!("Failed to find id for input: {s}"))?;
        let shown = shown_part
            .split("; ")
            .map(|shown_round| {
                let red = get_num(&RE_RED, shown_round)
                    .with_context(|| format!("Failed to get red for {shown_round}"))?;
                let green = get_num(&RE_GREEN, shown_round)
                    .with_context(|| format!("Failed to get green for {shown_round}"))?;
                let blue = get_num(&RE_BLUE, shown_round)
                    .with_context(|| format!("Failed to get blue for {shown_round}"))?;

                Ok(Rgb { red, green, blue })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Game { id, shown })
    }
}

fn parse(input: &str) -> Result<Vec<Game>> {
    input.lines().map(|s| s.parse()).collect()
}

fn p1(games: &[Game], max_red: u32, max_green: u32, max_blue: u32) -> usize {
    games
        .iter()
        .filter_map(|game| {
            game.shown
                .iter()
                .all(|Rgb { red, green, blue }| {
                    *red <= max_red && *green <= max_green && *blue <= max_blue
                })
                .then_some(game.id)
        })
        .sum()
}

fn p2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            game.shown
                .iter()
                .fold(Rgb::default(), |acc, other| acc.max(other))
                .power()
        })
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/2.txt").expect("input file should be there");
    let games = parse(&input)?;
    let start1 = Instant::now();
    let silver = p1(&games, 12, 13, 14);
    let end1 = Instant::now();
    println!("silver: {silver}");
    println!("took: {:?}", end1.duration_since(start1));
    let start2 = Instant::now();
    let gold = p2(&games);
    let end2 = Instant::now();
    println!("gold: {gold}");
    println!("took: {:?}", end2.duration_since(start2));
    Ok(())
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let games = parse(input)?;
        assert_eq!(p1(&games, 12, 13, 14), 8);
        Ok(())
    }
    #[test]
    fn sample2() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let games = parse(input)?;
        assert_eq!(p2(&games), 2286);
        Ok(())
    }
}

