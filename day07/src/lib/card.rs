use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, strum::EnumCount)]
pub enum Card {
    CardA = 13,
    CardK = 12,
    CardQ = 11,
    CardJ = 10,
    CardT = 9,
    Card9 = 8,
    Card8 = 7,
    Card7 = 6,
    Card6 = 5,
    Card5 = 4,
    Card4 = 3,
    Card3 = 2,
    Card2 = 1,
    CardG = 0, // joker case
}

impl Default for Card {
    fn default() -> Self {
        Card::Card2
    }
}

#[derive(Debug)]
pub enum CardParseError {
    UnknownCard(),
    WrongSize(),
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter = s.chars().next().ok_or(CardParseError::WrongSize())?;
        match letter {
            'A' => Ok(Card::CardA),
            'K' => Ok(Card::CardK),
            'Q' => Ok(Card::CardQ),
            'J' => Ok(Card::CardJ),
            'T' => Ok(Card::CardT),
            '9' => Ok(Card::Card9),
            '8' => Ok(Card::Card8),
            '7' => Ok(Card::Card7),
            '6' => Ok(Card::Card6),
            '5' => Ok(Card::Card5),
            '4' => Ok(Card::Card4),
            '3' => Ok(Card::Card3),
            '2' => Ok(Card::Card2),
            'G' => Ok(Card::CardG),
            _ => Err(CardParseError::UnknownCard()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Combination {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;

    #[test]
    fn test_card_comparing() {
        assert!(Card2 < CardA);
        assert!(CardK > Card3);
        assert!(CardT == CardT);
    }

    #[test]
    fn test_cards_sorting() {
        let mut hand = [Card2, CardA, CardT, Card2, Card6];

        let expected = [Card2, Card2, Card6, CardT, CardA];

        assert_ne!(hand, expected);

        hand.sort();
        assert_eq!(hand, expected);
    }
}
