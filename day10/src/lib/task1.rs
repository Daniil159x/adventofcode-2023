use std::collections::VecDeque;

static DIR_UP: usize = 0;
static DIR_DOWN: usize = 1;
static DIR_LEFT: usize = 2;
static DIR_RIGHT: usize = 3;

struct Node {
    number: usize,
    symbol: char,
    distance: Option<u32>,
}

fn get_direction(c: char) -> [bool; 4] {
    let mut res: [bool; 4] = [false; 4];

    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    match c {
        '|' => {
            res[DIR_UP] = true;
            res[DIR_DOWN] = true;
        }
        '-' => {
            res[DIR_LEFT] = true;
            res[DIR_RIGHT] = true;
        }
        'L' => {
            res[DIR_UP] = true;
            res[DIR_RIGHT] = true;
        }
        'J' => {
            res[DIR_UP] = true;
            res[DIR_LEFT] = true;
        }
        '7' => {
            res[DIR_LEFT] = true;
            res[DIR_DOWN] = true;
        }
        'F' => {
            res[DIR_RIGHT] = true;
            res[DIR_DOWN] = true;
        }
        'S' => {
            res[DIR_UP] = true;
            res[DIR_DOWN] = true;
            res[DIR_LEFT] = true;
            res[DIR_RIGHT] = true;
        }
        _ => {}
    };

    return res;
}

fn bfs(g: &mut Vec<Node>, start: usize, wight: usize) -> u32 {
    let mut queue = VecDeque::new();

    g[start].distance = Some(0);
    queue.push_back(start);

    let mut max_distance = 0;

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        let v_dirs = get_direction(g[v].symbol);
        let curr_distance = g[v].distance.unwrap();
        max_distance = max_distance.max(curr_distance);

        // up
        if v_dirs[DIR_UP]
            && v >= wight
            && g[v - wight].distance.is_none()
            && get_direction(g[v - wight].symbol)[DIR_DOWN]
        {
            g[v - wight].distance = Some(curr_distance + 1);
            queue.push_back(v - wight);
        }

        // down
        if v_dirs[DIR_DOWN]
            && v + wight < g.len()
            && g[v + wight].distance.is_none()
            && get_direction(g[v + wight].symbol)[DIR_UP]
        {
            g[v + wight].distance = Some(curr_distance + 1);
            queue.push_back(v + wight);
        }

        // left
        if v_dirs[DIR_LEFT]
            && v >= 1
            && g[v - 1].distance.is_none()
            && get_direction(g[v - 1].symbol)[DIR_RIGHT]
        {
            g[v - 1].distance = Some(curr_distance + 1);
            queue.push_back(v - 1);
        }

        // right
        if v_dirs[DIR_RIGHT]
            && v + 1 < g.len()
            && g[v + 1].distance.is_none()
            && get_direction(g[v + 1].symbol)[DIR_LEFT]
        {
            g[v + 1].distance = Some(curr_distance + 1);
            queue.push_back(v + 1);
        }
    }

    max_distance
}

pub fn solve(content: &str) -> u32 {
    let wight = content.find(|c| c == '\n').unwrap() + 1;

    let mut g = content
        .char_indices()
        .map(|(idx, c)| Node {
            number: idx,
            symbol: c,
            distance: None,
        })
        .collect::<Vec<_>>();

    let start_idx = g.iter().find(|n| n.symbol == 'S').unwrap().number;

    bfs(&mut g, start_idx, wight)
}
