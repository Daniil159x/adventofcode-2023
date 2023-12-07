use crate::{cubes::Cubes, game::Game};

pub fn solve(content: &str, max_bag: Cubes) -> u32 {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|x| x.parse::<Game>().unwrap())
        .filter(|x| x.bag <= max_bag)
        .map(|x| x.number)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_from_one_possible_game() {
        assert_eq!(
            solve("Game 1337: 1 red; 2 green; 3 blue", Cubes::new(12, 13, 14)),
            1337
        );
    }

    #[test]
    fn test_solve_from_two_possible_game() {
        let game = "
        Game 1000: 10 red; 2 green; 3 blue
        Game 337: 1 red; 12 green; 3 blue
        ";
        assert_eq!(solve(&game, Cubes::new(12, 13, 14)), 1337);
    }

    #[test]
    fn test_solve_from_one_impossible_game() {
        assert_eq!(
            solve(
                "Game 1337: 100 red; 2 green; 3 blue",
                Cubes::new(12, 13, 14)
            ),
            0
        );
    }

    #[test]
    fn test_solve_from_two_impossible_game() {
        let game = "
        Game 1000: 10 red; 20 green; 3 blue
        Game 337: 1 red; 12 green; 30 blue
        ";
        assert_eq!(solve(&game, Cubes::new(12, 13, 14)), 0);
    }

    #[test]
    fn test_solve_from_games() {
        let game = "
        Game 1000: 10 red; 2 green; 3 blue
        Game 10: 1 red; 12 green; 30 blue
        Game 337: 1 red; 12 green; 3 blue
        Game 11: 20 red; 12 green; 3 blue
        ";
        assert_eq!(solve(&game, Cubes::new(12, 13, 14)), 1337);
    }
}
