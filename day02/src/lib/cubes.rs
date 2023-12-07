use std::{num::ParseIntError, str::FromStr};

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn make_top(&self, other: Cubes) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn get_power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseCubesError {
    NoComma(),
    NoNumber(ParseIntError),
    UnknownColor(),
}

impl From<ParseIntError> for ParseCubesError {
    fn from(err: ParseIntError) -> ParseCubesError {
        ParseCubesError::NoNumber(err)
    }
}

impl FromStr for Cubes {
    type Err = ParseCubesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Cubes::default();

        for part in s.split(',') {
            let (num_str, name_str) = part
                .trim()
                .split_once(' ')
                .ok_or(ParseCubesError::NoComma())?;

            let num = num_str.parse::<u32>()?;
            match name_str {
                "red" => cubes.red = num,
                "green" => cubes.green = num,
                "blue" => cubes.blue = num,
                _ => return Err(ParseCubesError::UnknownColor()),
            }
        }
        Ok(cubes)
    }
}
