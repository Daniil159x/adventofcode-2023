use std::{num::ParseIntError, str::FromStr};

use crate::hand::{Hand, HandParseError};

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct Bit {
    pub hand: Hand,
    pub bit: u32,
}

impl PartialOrd for Bit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

#[derive(Debug)]
pub enum BitParseError {
    WrongFormat(),
    HandWrong(HandParseError),
    BitWrong(ParseIntError),
}

impl From<ParseIntError> for BitParseError {
    fn from(v: ParseIntError) -> Self {
        Self::BitWrong(v)
    }
}

impl From<HandParseError> for BitParseError {
    fn from(v: HandParseError) -> Self {
        Self::HandWrong(v)
    }
}

impl FromStr for Bit {
    type Err = BitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bit_str) = s.split_once(' ').ok_or(BitParseError::WrongFormat())?;

        let hand = hand_str.parse()?;
        let bit = bit_str.parse()?;

        Ok(Bit { hand, bit })
    }
}
