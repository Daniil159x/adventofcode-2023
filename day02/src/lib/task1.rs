fn game_round_parse(line: &str) -> Option<(u32, u32, u32)> {
    let mut red = 0u32;
    let mut green = 0u32;
    let mut blue = 0u32;
    for part in line.split(',') {
        let (num_str, name_str) = part.trim().split_once(' ')?;

        let num = num_str.parse::<u32>().map_or(None, |x| Some(x))?;
        match name_str {
            "red" => red = num,
            "green" => green = num,
            "blue" => blue = num,
            _ => return None,
        }
    }

    Some((red, green, blue))
}

fn game_parse(line: &str) -> Option<(u32, (u32, u32, u32))> {
    let (game_number_str, game_content_str) = line.split_once(':')?;
    let game_number = game_number_str
        .strip_prefix("Game ")?
        .parse::<u32>()
        .map_or(None, |x| Some(x))?; // TODO: remove map_or to ? operator after changing return type to Result

    let round_max = (0u32, 0u32, 0u32);

    let round_max = game_content_str
        .split(';')
        .filter_map(game_round_parse)
        .fold(round_max, |max, round| {
            (round.0.max(max.0), round.1.max(max.1), round.2.max(max.2))
        });

    Some((game_number, round_max))
}

pub fn solve(content: &str, red: u32, green: u32, blue: u32) -> u32 {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(game_parse)
        .filter(|(_, (r, g, b))| r <= &red && g <= &green && b <= &blue)
        .map(|(num, _)| num)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_round_parse_from_empty() {
        assert_eq!(game_round_parse(""), None);
    }

    #[test]
    fn test_game_round_parse_from_one_part() {
        assert_eq!(game_round_parse("1 red"), Some((1u32, 0u32, 0u32)));
        assert_eq!(game_round_parse("1 green"), Some((0u32, 1u32, 0u32)));
        assert_eq!(game_round_parse("1 blue"), Some((0u32, 0u32, 1u32)));
    }

    #[test]
    fn test_game_round_parse_from_two_parts() {
        assert_eq!(game_round_parse("1 red, 2 green"), Some((1u32, 2u32, 0u32)));
        assert_eq!(game_round_parse("1 red, 2 blue"), Some((1u32, 0u32, 2u32)));
        assert_eq!(game_round_parse("1 green, 2 red"), Some((2u32, 1u32, 0u32)));
        assert_eq!(
            game_round_parse("1 green, 2 blue"),
            Some((0u32, 1u32, 2u32))
        );
        assert_eq!(game_round_parse("1 blue, 2 red"), Some((2u32, 0u32, 1u32)));
        assert_eq!(
            game_round_parse("1 blue, 2 green"),
            Some((0u32, 2u32, 1u32))
        );
    }

    #[test]
    fn test_game_round_parse_from_three_parts() {
        assert_eq!(
            game_round_parse("1 red, 2 green, 3 blue"),
            Some((1u32, 2u32, 3u32))
        );
        assert_eq!(
            game_round_parse("1 red, 2 blue, 3 green"),
            Some((1u32, 3u32, 2u32))
        );
        assert_eq!(
            game_round_parse("1 green, 2 red, 3 blue"),
            Some((2u32, 1u32, 3u32))
        );
        assert_eq!(
            game_round_parse("1 green, 2 blue, 3 red"),
            Some((3u32, 1u32, 2u32))
        );
        assert_eq!(
            game_round_parse("1 blue, 2 red, 3 green"),
            Some((2u32, 3u32, 1u32))
        );
        assert_eq!(
            game_round_parse("1 blue, 2 green, 3 red"),
            Some((3u32, 2u32, 1u32))
        );
    }

    #[test]
    fn test_game_parse_from_empty() {
        assert_eq!(game_parse(""), None);
    }

    #[test]
    fn test_game_parse_from_one_round() {
        assert_eq!(
            game_parse("Game 1337: 1 red"),
            Some((1337u32, (1u32, 0u32, 0u32)))
        );
        assert_eq!(
            game_parse("Game 1337: 1 red, 2 green"),
            Some((1337u32, (1u32, 2u32, 0u32)))
        );
        assert_eq!(
            game_parse("Game 1337: 1 red, 2 green, 3 blue"),
            Some((1337u32, (1u32, 2u32, 3u32)))
        );
    }

    #[test]
    fn test_game_parse_from_two_rounds() {
        assert_eq!(
            game_parse("Game 1337: 1 red; 2 green; 3 blue"),
            Some((1337u32, (1u32, 2u32, 3u32)))
        );
        assert_eq!(
            game_parse("Game 1337: 1 red, 50 blue; 2 green, 13 red; 3 blue"),
            Some((1337u32, (13u32, 2u32, 50u32)))
        );
        assert_eq!(
            game_parse("Game 1337: 1 red; 10 red, 2 green, 40 blue; 1 red, 3 blue, 1 green"),
            Some((1337u32, (10u32, 2u32, 40u32)))
        );
    }

    #[test]
    fn test_solve_from_one_possible_game() {
        assert_eq!(solve("Game 1337: 1 red; 2 green; 3 blue", 12, 13, 14), 1337);
    }

    #[test]
    fn test_solve_from_two_possible_game() {
        let game = "
        Game 1000: 10 red; 2 green; 3 blue
        Game 337: 1 red; 12 green; 3 blue
        ";
        assert_eq!(solve(&game, 12, 13, 14), 1337);
    }

    #[test]
    fn test_solve_from_one_impossible_game() {
        assert_eq!(solve("Game 1337: 100 red; 2 green; 3 blue", 12, 13, 14), 0);
    }

    #[test]
    fn test_solve_from_two_impossible_game() {
        let game = "
        Game 1000: 10 red; 20 green; 3 blue
        Game 337: 1 red; 12 green; 30 blue
        ";
        assert_eq!(solve(&game, 12, 13, 14), 0);
    }

    #[test]
    fn test_solve_from_games() {
        let game = "
        Game 1000: 10 red; 2 green; 3 blue
        Game 10: 1 red; 12 green; 30 blue
        Game 337: 1 red; 12 green; 3 blue
        Game 11: 20 red; 12 green; 3 blue
        ";
        assert_eq!(solve(&game, 12, 13, 14), 1337);
    }
}
