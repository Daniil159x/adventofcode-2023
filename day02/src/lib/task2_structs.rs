use crate::game::Game;

pub fn solve(content: &str) -> u32 {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|x| x.parse::<Game>().unwrap().bag.get_power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_from_one_game() {
        assert_eq!(solve("Game 1337: 1 red; 2 green; 3 blue"), 6);
    }

    #[test]
    fn test_solve_from_two_game() {
        let game = "
        Game 1000: 10 red; 2 green; 3 blue
        Game 337: 1 red; 12 green; 3 blue
        ";
        assert_eq!(solve(&game), 60 + 36);
    }

    #[test]
    fn test_solve_from_games() {
        let game = "
        Game 1000: 10 red; 2 green; 3 blue
        Game 10: 1 red; 12 green; 30 blue
        Game 337: 1 red; 12 green; 3 blue
        Game 11: 20 red; 12 green; 3 blue
        ";
        assert_eq!(solve(&game), 60 + 360 + 36 + 720);
    }
}
