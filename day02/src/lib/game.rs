use std::{num::ParseIntError, str::FromStr};

use crate::cubes::{Cubes, ParseCubesError};

#[derive(Default)]
pub struct Game {
    pub number: u32,
    pub bag: Cubes,
}

#[derive(Debug)]
pub enum GameParseError {
    NoColon(),
    NoGamePrefix(),
    NoNumber(ParseIntError),
    CubesParse(ParseCubesError),
}

impl From<ParseCubesError> for GameParseError {
    fn from(v: ParseCubesError) -> Self {
        Self::CubesParse(v)
    }
}

impl From<ParseIntError> for GameParseError {
    fn from(v: ParseIntError) -> Self {
        Self::NoNumber(v)
    }
}

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_number_str, game_content_str) =
            s.trim().split_once(':').ok_or(GameParseError::NoColon())?;

        let number = game_number_str
            .strip_prefix("Game ")
            .ok_or(GameParseError::NoGamePrefix())?
            .parse::<u32>()?;

        // NOTE: very helpfully with error handling https://stackoverflow.com/a/63120052
        let bag = game_content_str.split(';').try_fold(
            Cubes::default(),
            |acc, x| -> Result<Cubes, ParseCubesError> { Ok(acc.make_top(x.parse()?)) },
        )?;

        Ok(Game { number, bag })
    }
}
