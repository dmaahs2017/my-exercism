use std::convert::TryFrom;

pub type Rank = u8;

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn from_str(c: &str) -> Card {
        let (rank, suit) = c.split_at(c.len() - 1);

        let suit =
            Suit::try_from(suit.chars().last().expect("idek yet")).expect("Not a valid suit");
        let rank = if let Ok(d) = rank.parse::<Rank>() {
            if d >= 1 && d <= 13 {
                d
            } else {
                panic!(format!("Invalid rank: {}", d))
            }
        } else if let Ok(c) = rank.parse::<char>() {
            match c {
                'A' => 14,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                _ => panic!(format!("Invalid rank: {}", c)),
            }
        } else {
            panic!("Not a valid rank")
        };
        Card { rank, suit }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl TryFrom<char> for Suit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'H' => Ok(Suit::Heart),
            'S' => Ok(Suit::Spade),
            'C' => Ok(Suit::Club),
            'D' => Ok(Suit::Diamond),
            _ => Err("Invalid char conversion to suit."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suit_from_char_works() {
        assert_eq!(Suit::try_from('D').unwrap(), Suit::Diamond);
        assert_eq!(Suit::try_from('H').unwrap(), Suit::Heart);
        assert_eq!(Suit::try_from('C').unwrap(), Suit::Club);
        assert_eq!(Suit::try_from('S').unwrap(), Suit::Spade);
    }

    #[test]
    fn card_from_str_works() {
        let test =
            |c: &str, d: Rank, s: Suit| assert_eq!(Card { rank: d, suit: s }, Card::from_str(c));

        test("10C", 10, Suit::Club);
        test("AD", 14, Suit::Diamond);
        test("JS", 11, Suit::Spade);
        test("5H", 5, Suit::Heart);
    }
}
