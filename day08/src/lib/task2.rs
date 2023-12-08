use num::Integer;

use crate::network::Network;

fn travel(net: &Network, start: &str) -> u64 {
    let mut curr_node = start;

    let mut steps = 0u64;
    for c in net.path.chars().cycle() {
        steps += 1;
        let next_node = match c {
            'L' => net.map[curr_node].0,
            'R' => net.map[curr_node].1,
            _ => panic!(),
        };

        if next_node.chars().last().unwrap() == 'Z' {
            break;
        }

        curr_node = next_node;
    }
    steps
}

pub fn solve(content: &str) -> u64 {
    let net: Network = content.try_into().unwrap();

    net.map
        .keys()
        .filter(|x| x.chars().last().unwrap() == 'A')
        .map(|x| travel(&net, x))
        // https://stackoverflow.com/a/147523
        // lcm(a,b,c) = lcm(lcm(a, b), c)
        .fold(1u64, |acc, x| acc.lcm(&x))
}
