fn is_detail(c: char) -> bool {
    c != '.' && !c.is_ascii_whitespace() && !c.is_ascii_digit()
}

fn is_part(table: &str, column_size: usize, index: usize, number_len: usize) -> Option<u32> {
    let num_str = &table[index..index + number_len];
    let number = num_str.parse::<u32>().unwrap();

    // strong over number
    if index >= column_size
        && table[(index - column_size)..]
            .chars()
            .take(number_len as usize)
            .any(is_detail)
    {
        return Some(number);
    }

    // strong under number
    if index + column_size < table.len()
        && table[(index + column_size)..]
            .chars()
            .take(number_len as usize)
            .any(is_detail)
    {
        return Some(number);
    }

    let s_index = index as isize;
    let s_number_len = number_len as isize;
    let s_column_size = column_size as isize;

    let before = s_index - 1;
    let above_before = before - s_column_size;
    let below_before = before + s_column_size;

    let after = s_index + s_number_len;
    let above_after = after - s_column_size;
    let below_after = after + s_column_size;

    let sides = [
        before,
        above_before,
        below_before,
        after,
        above_after,
        below_after,
    ];
    if sides
        .iter()
        .filter(|x| 0 <= **x && **x < (table.len() as isize))
        .any(|x| is_detail(table.chars().nth(*x as usize).unwrap()))
    {
        return Some(number);
    }

    None
}

pub fn solve(content: &str) -> u32 {
    let column = content.lines().next().unwrap().len() + 1;

    content
        .char_indices()
        .scan(0 as usize, |digits, (i, c)| {
            let is_c_num = c.is_ascii_digit();
            // number yet
            if *digits > 0 && is_c_num {
                *digits += 1;
                return Some((false, 0, 0));
            }

            // number ended
            if *digits > 0 && !is_c_num {
                let count = *digits;
                *digits = 0;
                return Some((true, i - count, count));
            }

            // number's begun
            if *digits == 0 && is_c_num {
                *digits = 1;
                return Some((false, 0, 0));
            }

            // no number
            Some((false, 0, 0))
        })
        .filter_map(|(flag, index, count)| if flag { Some((index, count)) } else { None })
        .filter_map(|(index, count)| is_part(content, column, index, count))
        .sum()
}
