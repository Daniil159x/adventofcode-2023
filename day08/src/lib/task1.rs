use crate::network::Network;

pub fn solve(content: &str) -> u32 {
    let net: Network = content.try_into().unwrap();

    let mut curr_node = "AAA";
    let mut steps = 0u32;
    for c in net.path.chars().cycle() {
        steps += 1;
        let next_node = match c {
            'L' => net.map[curr_node].0,
            'R' => net.map[curr_node].1,
            _ => panic!(),
        };

        if next_node == "ZZZ" {
            break;
        }

        curr_node = next_node;
    }
    steps
}
