fn extract_num(table: &str, index: usize) -> u32 {
    let start_index = table[..index].rfind(|x: char| !x.is_ascii_digit());
    let start_index = match start_index {
        Some(index_non_digit) => index_non_digit + 1,
        None => 0,
    };

    let end_num = table[index..]
        .find(|x: char| !x.is_ascii_digit())
        .or(Some(1))
        .unwrap();
    let end_index = index + end_num;

    let num_str = &table[start_index..end_index];
    num_str.parse().unwrap()
}

fn extract_gear(table: &str, index: usize, column_size: usize) -> Option<u32> {
    let i_index = index as isize;
    let i_column_size = column_size as isize;

    let bounded = |i: isize| 0 <= i && i < (table.len() as isize);
    let is_num = |i: isize| bounded(i) && table.chars().nth(i as usize).unwrap().is_ascii_digit();

    let before = i_index - 1;
    let above_before = before - i_column_size;
    let below_before = before + i_column_size;

    let after = i_index + 1;
    let above_after = after - i_column_size;
    let below_after = after + i_column_size;

    let below = i_index + i_column_size;
    let above = i_index - i_column_size;

    let circle = [
        before,
        -1, // fake end number
        after,
        -1,
        above_before,
        above,
        above_after,
        -1,
        below_before,
        below,
        below_after,
    ];

    let num_idx = circle
        .iter()
        .scan(false, |prev_is_num, index| {
            let curr_is_num = is_num(*index);

            // new number
            if !*prev_is_num && curr_is_num {
                *prev_is_num = true;
                return Some((true, index));
            }

            // number yet
            if *prev_is_num && curr_is_num {
                return Some((false, index));
            }

            // number ended
            if *prev_is_num && !curr_is_num {
                *prev_is_num = false;
                return Some((false, index));
            }

            // there's no number
            Some((false, index))
        })
        .filter_map(|x| if x.0 { Some(x.1) } else { None })
        .copied()
        .collect::<Vec<_>>();

    if num_idx.len() != 2 {
        return None;
    }

    let prod = num_idx
        .iter()
        .map(|index| extract_num(table, *index as usize))
        .product();
    Some(prod)
}

pub fn solve(content: &str) -> u32 {
    let column = content.lines().next().unwrap().len() + 1;

    dbg!(&content);

    content
        .char_indices()
        .filter(|(_, c)| *c == '*')
        .filter_map(|(index, _)| extract_gear(content, index, column))
        .sum()
}
