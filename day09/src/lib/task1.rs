use crate::predict::predict_next;

fn predict_for_line(line: &str) -> i64 {
    let a = line
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    predict_next(&a)
}

pub fn solve(content: &str) -> i64 {
    content
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| predict_for_line(line))
        .sum()
}
