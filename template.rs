use std::time::Instant;

fn p1<T>(_: T) -> usize {
    0
}

fn p2<T>(_: T) -> usize {
    0
}

fn main() {
    let input = include_str!("../../input/n.txt");
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

#[cfg(test)]
mod test_dayn {
    use super::*;

    #[test]
    fn sample1() {}
    #[test]
    fn sample2() {}
}

