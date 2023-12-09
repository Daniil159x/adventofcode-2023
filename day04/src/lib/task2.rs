use std::collections::HashSet;

fn calc_point(line: &str) -> usize {
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

    count
}

pub fn solve(content: &str) -> u32 {
    let mut array = content
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| (calc_point(line), 1u32))
        .collect::<Vec<_>>();

    for i in 0..array.len() {
        let (wins, count) = array[i];
        for j in 0..wins {
            array[i + j + 1].1 += count;
        }
    }

    array.iter().map(|x| x.1).sum()
}
