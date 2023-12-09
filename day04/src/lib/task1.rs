use std::collections::HashSet;

fn calc_point(line: &str) -> u32 {
    let (_, numbers) = line.split_once(':').unwrap();
    let (wining_str, numbers_str) = numbers.split_once('|').unwrap();

    let wins = wining_str
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<HashSet<u32>>();

    let count = numbers_str
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .filter(|num| wins.contains(num))
        .count();

    if count == 0 {
        return 0;
    }

    2u32.pow((count - 1) as u32)
}

pub fn solve(content: &str) -> u32 {
    content
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| calc_point(line))
        .sum()
}
