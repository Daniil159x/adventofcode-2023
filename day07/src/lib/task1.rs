use crate::bit::Bit;

pub fn solve(content: &str) -> u32 {
    let mut bits = content
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<Bit>().unwrap())
        .collect::<Vec<_>>();

    bits.sort();

    bits.iter()
        .enumerate()
        .map(|(i, bit)| (i as u32 + 1) * bit.bit)
        .sum()
}
