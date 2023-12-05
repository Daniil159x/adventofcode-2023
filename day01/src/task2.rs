fn to_digit(s: &str) -> Option<u32> {
    let digit = s.chars().next()?.to_digit(10);
    if digit.is_some() {
        return digit;
    }

    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let digit = words
        .iter()
        .enumerate()
        .find(|digit| s.starts_with(digit.1));

    match digit {
        Some((num, _)) => Some(1 + num as u32),
        None => None,
    }
}

fn extract_number(line: &str) -> Option<u32> {
    let size = line.len();

    let first = (0..size).find_map(|offset| to_digit(&line[offset..]))?;
    let second = (0..size)
        .rev()
        .find_map(|offset| to_digit(&line[offset..]))?;

    Some(first * 10 + second)
}

pub fn solve(content: &str) -> u32 {
    content
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(extract_number)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digit_from_empty() {
        assert_eq!(to_digit(""), None);
    }

    #[test]
    fn test_to_digit_from_digit() {
        assert_eq!(to_digit("1"), Some(1));
        assert_eq!(to_digit("2"), Some(2));
        assert_eq!(to_digit("3"), Some(3));
        assert_eq!(to_digit("4"), Some(4));
        assert_eq!(to_digit("5"), Some(5));
        assert_eq!(to_digit("6"), Some(6));
        assert_eq!(to_digit("7"), Some(7));
        assert_eq!(to_digit("8"), Some(8));
        assert_eq!(to_digit("9"), Some(9));
    }

    #[test]
    fn test_to_digit_from_digit_with_prefix() {
        assert_eq!(to_digit("ffweonefew1"), None);
    }

    #[test]
    fn test_to_digit_from_nondigit() {
        assert_eq!(to_digit("ergwegwergweewcew"), None);
    }

    #[test]
    fn test_to_digit_from_word() {
        assert_eq!(to_digit("one"), Some(1));
        assert_eq!(to_digit("two"), Some(2));
        assert_eq!(to_digit("three"), Some(3));
        assert_eq!(to_digit("four"), Some(4));
        assert_eq!(to_digit("five"), Some(5));
        assert_eq!(to_digit("six"), Some(6));
        assert_eq!(to_digit("seven"), Some(7));
        assert_eq!(to_digit("eight"), Some(8));
        assert_eq!(to_digit("nine"), Some(9));
    }

    #[test]
    fn test_extract_number_from_empty() {
        assert_eq!(extract_number(""), None);
    }

    #[test]
    fn test_extract_number_from_digit() {
        assert_eq!(extract_number("ergw1egwergw3eewcew"), Some(13));
    }

    #[test]
    fn test_extract_number_from_words() {
        assert_eq!(extract_number("ergwoneegwergwthreeeewcew"), Some(13));
    }

    #[test]
    fn test_extract_number_from_words_and_digits() {
        assert_eq!(extract_number("erg5woneegwergwthreeeewc9ew"), Some(59));
        assert_eq!(extract_number("erg5woneegwergw9threeeewcew"), Some(53));
        assert_eq!(extract_number("ergwone5egwergwthreeeewc9ew"), Some(19));
        assert_eq!(extract_number("ergwone5egwergw9threeeewcew"), Some(13));
    }

    #[test]
    fn test_solve_empty() {
        assert_eq!(solve(""), 0);
    }

    #[test]
    fn test_solve_simple() {
        let content = "269four1bzjrmheight3fgcr\n9seventhreefourzlqgjnrmq7xcmntjncntsixthlgq\nseveneight19fbphndppb8twonebbj\n\n\n";
        assert_eq!(solve(content), 23 + 96 + 71);
    }
}
