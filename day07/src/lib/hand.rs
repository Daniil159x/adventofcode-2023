use std::{cmp::Reverse, str::FromStr};

use strum::EnumCount;

use crate::card::{Card, CardParseError, Combination};

pub type Cards = [Card; 5];

#[derive(PartialEq, Eq, Ord, Debug)]
pub struct Hand {
    hand: Cards,
    cached_combination: Combination,
}

impl Hand {
    pub fn new(hand: Cards) -> Hand {
        let mut ret = Hand {
            hand,
            cached_combination: Combination::HighCard,
        };
        ret.cached_combination = ret.calc_combination();
        ret
    }

    pub fn replace_jacket_to_joker(&mut self) {
        for ele in self.hand.as_mut() {
            if *ele == Card::CardJ {
                *ele = Card::CardG;
            }
        }
        self.cached_combination = self.calc_combination();
    }

    fn calc_combination(&self) -> Combination {
        let mut arr_hash = [0u8; Card::COUNT];

        for card in self.hand {
            let card_idx = card as usize;
            arr_hash[card_idx] += 1;
        }

        let jokers = arr_hash[Card::CardG as usize];
        arr_hash[Card::CardG as usize] = 0;

        // descending
        arr_hash.sort_by_key(|x| Reverse(*x));

        // AAAAA
        // AAAGG
        if arr_hash[0] + jokers == 5 {
            return Combination::FiveOfAKind;
        }

        // AAAA2
        // AAGG2
        if arr_hash[0] + jokers == 4 {
            return Combination::FourOfAKind;
        }

        // AAA22
        // AA2GG
        if arr_hash[0] + arr_hash[1] + jokers == 5 {
            return Combination::FullHouse;
        }

        // AAA52
        // AA52G
        if arr_hash[0] + jokers == 3 {
            return Combination::ThreeOfAKind;
        }

        // AA552
        // AA52G
        // A52GG
        if arr_hash[0] + arr_hash[1] + jokers == 4 {
            return Combination::TwoPair;
        }

        // AA952
        // A952G
        if arr_hash[0] + jokers == 2 {
            return Combination::OnePair;
        }

        assert_eq!(arr_hash[0] + jokers, 1);

        Combination::HighCard
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self
            .cached_combination
            .partial_cmp(&other.cached_combination)
        {
            Some(core::cmp::Ordering::Equal) => self.hand.partial_cmp(&other.hand),
            ord => ord,
        }
    }
}

#[derive(Debug)]
pub enum HandParseError {
    WrongCard(CardParseError),
    InsufficientSize(),
}

impl From<CardParseError> for HandParseError {
    fn from(v: CardParseError) -> Self {
        Self::WrongCard(v)
    }
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err(HandParseError::InsufficientSize());
        }

        let mut hand = Cards::default();
        for i in 0..5 {
            hand[i] = s[i..i + 1].parse()?;
        }

        Ok(Hand::new(hand))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card::*;

    #[test]
    fn test_hand_parse() {
        assert_eq!(
            "32T3K".parse::<Hand>().unwrap().hand,
            [Card3, Card2, CardT, Card3, CardK]
        );

        assert_eq!(
            "T55J5".parse::<Hand>().unwrap().hand,
            [CardT, Card5, Card5, CardJ, Card5]
        );

        assert_eq!(
            "KK677".parse::<Hand>().unwrap().hand,
            [CardK, CardK, Card6, Card7, Card7]
        );

        assert_eq!(
            "KTJJT".parse::<Hand>().unwrap().hand,
            [CardK, CardT, CardJ, CardJ, CardT]
        );

        assert_eq!(
            "QQQJA".parse::<Hand>().unwrap().hand,
            [CardQ, CardQ, CardQ, CardJ, CardA]
        );
    }

    #[test]
    fn test_hand_comparing() {
        let hand1 = "32T3K".parse::<Hand>().unwrap();
        let hand2 = "KTJJT".parse::<Hand>().unwrap();
        let hand3 = "KK677".parse::<Hand>().unwrap();
        let hand4 = "T55J5".parse::<Hand>().unwrap();
        let hand5 = "QQQJA".parse::<Hand>().unwrap();

        assert!(hand1 < hand2);
        assert!(hand2 < hand3);
        assert!(hand4 < hand5);
        assert!(hand1 < hand5);

        assert!(hand5 > hand4);
        assert!(hand4 > hand3);
        assert!(hand3 > hand2);
        assert!(hand2 > hand1);

        let hand1_copy = "32T3K".parse::<Hand>().unwrap();
        assert!(hand1 == hand1_copy);
    }

    #[test]
    fn test_hand_sorting() {
        let hand1 = "32T3K".parse::<Hand>().unwrap();
        let hand2 = "T55J5".parse::<Hand>().unwrap();
        let hand3 = "KK677".parse::<Hand>().unwrap();
        let hand4 = "KTJJT".parse::<Hand>().unwrap();
        let hand5 = "QQQJA".parse::<Hand>().unwrap();

        let mut arr = [&hand1, &hand2, &hand3, &hand4, &hand5];
        arr.sort();

        assert_eq!(arr, [&hand1, &hand4, &hand3, &hand2, &hand5]);
    }
}
