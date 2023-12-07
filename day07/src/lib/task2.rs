use crate::bit::Bit;

fn parse_bit_with_joker(s: &str) -> Bit {
    let mut bit = s.parse::<Bit>().unwrap();
    bit.hand.replace_jacket_to_joker();
    bit
}

pub fn solve(content: &str) -> u32 {
    let mut bits = content
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(parse_bit_with_joker)
        .collect::<Vec<_>>();

    bits.sort();

    bits.iter()
        .enumerate()
        .map(|(i, bit)| (i as u32 + 1) * bit.bit)
        .sum()
}
