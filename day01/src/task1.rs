fn extract_number(line: &str) -> Option<u32> {
    let first = line.chars().find(char::is_ascii_digit)?.to_digit(10)?;
    let second = line.chars().rfind(char::is_ascii_digit)?.to_digit(10)?;

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
    fn test_extract_number_from_empty() {
        assert_eq!(extract_number(""), None);
    }

    #[test]
    fn test_extract_number_2_digit() {
        assert_eq!(extract_number("aaa1bbb3ccc"), Some(13));
    }

    #[test]
    fn test_extract_number_4_digit() {
        assert_eq!(extract_number("aaa1bbb3ccc3ddd7eee"), Some(17));
    }

    #[test]
    fn test_solve_empty() {
        assert_eq!(solve(""), 0);
    }

    #[test]
    fn test_solve_simple() {
        let content = "aaa1bbb3ccc\naaa1bbb3ccc3ddd7eee\n\n\n";
        assert_eq!(solve(content), 13 + 17);
    }
}
