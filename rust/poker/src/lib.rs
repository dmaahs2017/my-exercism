use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    get_best_hands(hands.iter().map(|h| PokerHand::from_str(h)).collect())
        .map(|hands| hands.iter().map(|h| h.get_str()).collect())
}

fn get_best_hands(mut hands: Vec<PokerHand>) -> Option<Vec<PokerHand>> {
    hands.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let first = hands.first().unwrap();
    Some(hands.iter().take_while(|&h| h.eq(first)).cloned().collect())
}

type Rank = u8;

// this derive may not work
#[derive(Debug)]
enum Score {
    FiveOfAKind,
    StraightFlush(Rank),
    FourOfAKind(Rank, Rank), // (FoAK, Kicker)
    FullHouse(Rank, Rank),
    Flush(Vec<Rank>),
    Straight(Rank),
    ThreeOfAKind(Rank, Rank, Rank), // (ThreeOfAKind, Kicker, Kicker)
    TwoPair(Rank, Rank, Rank),      //(Pair, Pair, Kicker)
    OnePair(Rank),
    HighCard(Vec<Rank>),
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self {
            Self::FiveOfAKind => match other {
                Self::FiveOfAKind => Some(cmp::Ordering::Equal),
                _ => Some(cmp::Ordering::Greater),
            },
            Self::StraightFlush(sr) => match other {
                Self::FiveOfAKind => Some(cmp::Ordering::Less),
                Self::StraightFlush(or) => sr.partial_cmp(or),
                _ => Some(cmp::Ordering::Greater),
            },
            Self::FourOfAKind(sr, sk) => match other {
                Self::FiveOfAKind | Self::StraightFlush(_) => Some(cmp::Ordering::Less),
                Self::FourOfAKind(or, ok) => {
                    if sr == or {
                        sk.partial_cmp(ok)
                    } else {
                        sr.partial_cmp(or)
                    }
                }
                _ => Some(cmp::Ordering::Greater),
            },
            Self::FullHouse(sr1, sr2) => match other {
                Self::FiveOfAKind | Self::StraightFlush(_) | Self::FourOfAKind(_, _) => {
                    Some(cmp::Ordering::Less)
                }
                Self::FullHouse(or1, or2) => {
                    if sr1 == or1 {
                        sr2.partial_cmp(or2)
                    } else {
                        sr1.partial_cmp(or1)
                    }
                }
                _ => Some(cmp::Ordering::Greater),
            },
            Self::Flush(sr) => match other {
                Self::FiveOfAKind
                | Self::StraightFlush(_)
                | Self::FourOfAKind(_, _)
                | Self::FullHouse(_, _) => Some(cmp::Ordering::Less),
                Self::Flush(or) => sr.partial_cmp(or),
                _ => Some(cmp::Ordering::Greater),
            },
            Self::Straight(sr) => match other {
                Self::FiveOfAKind
                | Self::StraightFlush(_)
                | Self::FourOfAKind(_, _)
                | Self::FullHouse(_, _)
                | Self::Flush(_) => Some(cmp::Ordering::Less),
                Self::Straight(or) => sr.partial_cmp(or),
                _ => Some(cmp::Ordering::Greater),
            },
            Self::ThreeOfAKind(sr, sk1, sk2) => match other {
                Self::FiveOfAKind
                | Self::StraightFlush(_)
                | Self::FourOfAKind(_, _)
                | Self::FullHouse(_, _)
                | Self::Flush(_)
                | Self::Straight(_) => Some(cmp::Ordering::Less),
                Self::ThreeOfAKind(or, ok1, ok2) => {
                    if sr == or {
                        if sk1.max(sk2) == ok1.max(ok2) {
                            sk1.min(sk2).partial_cmp(ok1.min(ok2))
                        } else {
                            sk1.max(sk2).partial_cmp(ok1.max(ok2))
                        }
                    } else {
                        sr.partial_cmp(or)
                    }
                }
                _ => Some(cmp::Ordering::Greater),
            },
            Self::TwoPair(sr1, sr2, sk) => match other {
                Self::FiveOfAKind
                | Self::StraightFlush(_)
                | Self::FourOfAKind(_, _)
                | Self::FullHouse(_, _)
                | Self::Flush(_)
                | Self::Straight(_)
                | Self::ThreeOfAKind(_, _, _) => Some(cmp::Ordering::Less),
                Self::TwoPair(or1, or2, ok) => {
                    if sr1.max(sr2) == or1.max(or2) {
                        if sr1.min(sr2) == or1.min(or2) {
                            sk.partial_cmp(ok)
                        } else {
                            sr1.min(sr2).partial_cmp(or1.min(or2))
                        }
                    } else {
                        sr1.max(sr2).partial_cmp(or1.max(or2))
                    }
                }
                _ => Some(cmp::Ordering::Greater),
            },
            Self::OnePair(sr) => match other {
                Self::FiveOfAKind
                | Self::StraightFlush(_)
                | Self::FourOfAKind(_, _)
                | Self::FullHouse(_, _)
                | Self::Flush(_)
                | Self::Straight(_)
                | Self::ThreeOfAKind(_, _, _)
                | Self::TwoPair(_, _, _) => Some(cmp::Ordering::Less),
                Self::OnePair(or) => sr.partial_cmp(or),
                _ => Some(cmp::Ordering::Greater),
            },
            Self::HighCard(sranks) => match other {
                Self::HighCard(oranks) => sranks.partial_cmp(&oranks),
                _ => Some(cmp::Ordering::Less),
            },
        }
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::FiveOfAKind => match other {
                Self::FiveOfAKind => true,
                _ => false,
            },
            Self::StraightFlush(sr) => match other {
                Self::StraightFlush(or) => sr == or,
                _ => false,
            },
            Self::FourOfAKind(sr, sk) => match other {
                Self::FourOfAKind(or, ok) => sr == or && sk == ok,
                _ => false,
            },
            Self::FullHouse(sr1, sr2) => match other {
                Self::FullHouse(or1, or2) => sr1 == or1 && sr2 == or2,
                _ => false,
            },
            Self::Flush(sr) => match other {
                Self::Flush(or) => sr == or,
                _ => false,
            },
            Self::Straight(sr) => match other {
                Self::Straight(or) => sr == or,
                _ => false,
            },
            Self::ThreeOfAKind(sr, sk1, sk2) => match other {
                Self::ThreeOfAKind(or, ok1, ok2) => {
                    sr == or && sk1.max(sk2) == ok1.max(ok2) && sk1.min(sk2) == ok1.min(ok2)
                }
                _ => false,
            },
            Self::TwoPair(sr1, sr2, sk) => match other {
                Self::TwoPair(or1, or2, ok) => {
                    sr1.max(sr2) == or1.max(or2) && sr1.min(sr2) == or1.min(or2) && sk == ok
                }
                _ => false,
            },
            Self::OnePair(sr) => match other {
                Self::OnePair(or) => sr == or,
                _ => false,
            },
            Self::HighCard(sr) => match other {
                Self::HighCard(or) => sr == or,
                _ => false,
            },
        }
    }
}

#[derive(Clone, Debug)]
struct PokerHand<'a> {
    hand: Vec<Card>,
    str_hand: &'a str,
}

impl<'a> PokerHand<'a> {
    fn from_str(str_hand: &'a str) -> Self {
        PokerHand {
            hand: str_hand
                .split_whitespace()
                .map(|c| Card::from_str(c))
                .collect(),
            str_hand,
        }
    }

    fn get_str(&self) -> &'a str {
        self.str_hand
    }

    fn score_hand(&self) -> Score {
        let (four_of_a_kind, fours_kickers) = self.get_pairs_of_n(4);
        let (three_of_a_kind, threes_kickers) = self.get_pairs_of_n(3);
        let (pairs, pair_kickers) = self.get_pairs_of_n(2);
        let high_cards = self.get_ranks_sorted_high_to_low();

        if let Some(rank) = self.get_straight_flush() {
            return Score::StraightFlush(rank);
        }

        if let Some(&rank) = four_of_a_kind.first() {
            return Score::FourOfAKind(rank, *fours_kickers.first().unwrap());
        }

        if let (Some(&threes_rank), Some(&pair_rank)) = (three_of_a_kind.first(), pairs.first()) {
            return Score::FullHouse(threes_rank, pair_rank);
        }

        if let Some(ranks) = self.get_flush() {
            return Score::Flush(ranks);
        }

        if let Some(rank) = self.get_straight() {
            return Score::Straight(rank);
        }

        if let Some(&rank) = three_of_a_kind.first() {
            return Score::ThreeOfAKind(
                rank,
                *threes_kickers.first().unwrap(),
                *threes_kickers.last().unwrap(),
            );
        }

        if pairs.len() == 2 {
            return Score::TwoPair(
                *pairs.first().unwrap(),
                *pairs.last().unwrap(),
                *pair_kickers.first().unwrap(),
            );
        } else if let Some(pair) = pairs.first() {
            return Score::OnePair(*pair);
        }

        return Score::HighCard(high_cards);
    }

    fn get_straight_flush(&self) -> Option<Rank> {
        if self.is_flush() {
            self.get_straight()
        } else {
            None
        }
    }

    fn is_flush(&self) -> bool {
        let suit = self.hand.first().unwrap().suit;
        self.hand
            .iter()
            .map(|c| c.suit)
            .fold(true, |mut is_flush, s| {
                if s != suit {
                    is_flush = false;
                }
                is_flush
            })
    }

    fn get_flush(&self) -> Option<Vec<Rank>> {
        if self.is_flush() {
            Some(self.get_ranks_sorted_high_to_low())
        } else {
            None
        }
    }

    fn get_straight(&self) -> Option<Rank> {
        let mut prev: Option<Rank> = None;
        let mut is_straight = true;
        let ranks = self.get_ranks_sorted_high_to_low();
        for &rank in ranks.iter().as_ref() {
            dbg!(rank, prev);
            if let Some(prev) = prev {
                if rank + 1 != prev && !(prev == 14 && rank == 5) {
                    is_straight = false;
                    break;
                }
            }
            prev = Some(rank)
        }

        if is_straight {
            ranks.first().map(|&d| if d == 14 { 5 } else { d })
        } else {
            None
        }
    }

    fn get_ranks_sorted_high_to_low(&self) -> Vec<Rank> {
        let mut ranks: Vec<_> = self.hand.iter().map(|c| c.rank).collect();

        ranks.sort_unstable_by(|a, b| b.cmp(a));

        ranks
    }

    fn get_pairs_of_n(&self, n: u8) -> (Vec<Rank>, Vec<Rank>) {
        let pairs = self.get_pairs_of_n_helper(n);
        let kickers = self
            .hand
            .iter()
            .filter_map(|c| {
                if !pairs.contains(&c.rank) {
                    Some(c.rank)
                } else {
                    None
                }
            })
            .collect();
        (pairs.iter().map(|&n| n).collect(), kickers)
    }

    fn get_pairs_of_n_helper(&self, n: u8) -> HashSet<Rank> {
        self.hand
            .iter()
            .map(|c| c.rank)
            .fold(HashMap::new(), |mut map, rank| {
                *map.entry(rank).or_insert(0) += 1;
                map
            })
            .iter()
            .filter_map(|(&rank, &count)| if count == n { Some(rank) } else { None })
            .collect()
    }

    #[cfg(test)]
    fn is_same_hand(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl<'a> cmp::PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.score_hand() == other.score_hand()
    }
}

impl<'a> cmp::PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.score_hand().partial_cmp(&other.score_hand())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Suit {
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

#[derive(Debug, PartialEq, Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn from_str(c: &str) -> Card {
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hashset {
        ( $( $v:expr ),* ) => {
            {
                #[allow(unused_mut)]
                let mut hs = ::std::collections::HashSet::new();
                $(
                    hs.insert($v);
                )*
                hs
            }
        };
        ( $( $k:expr => $v:expr, )+ ) => {
            macros::hashset!($($v),+)
        };
    }

    #[test]
    fn suit_from_char_works() {
        assert_eq!(Suit::try_from('D').unwrap(), Suit::Diamond);
        assert_eq!(Suit::try_from('H').unwrap(), Suit::Heart);
        assert_eq!(Suit::try_from('C').unwrap(), Suit::Club);
        assert_eq!(Suit::try_from('S').unwrap(), Suit::Spade);
    }

    #[test]
    fn poker_hand_get_str_works() {
        let h = "2H 2H 2H 2H 2H";
        let ph = PokerHand::from_str(h);
        assert_eq!(ph.get_str(), h)
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

    #[test]
    fn poker_hand_from_str_works() {
        let h = "2H 3D 10S JC KS";
        let ph = PokerHand::from_str("2H 3D 10S JC KS");
        assert!(ph.is_same_hand(&PokerHand {
            hand: vec![
                Card::from_str("2H"),
                Card::from_str("3D"),
                Card::from_str("10S"),
                Card::from_str("JC"),
                Card::from_str("KS"),
            ],
            str_hand: h,
        }))
    }

    #[test]
    fn get_ranks_works() {
        let h = PokerHand::from_str("2H KH 10C JD KC");
        assert_eq!(h.get_ranks_sorted_high_to_low(), vec![13, 13, 11, 10, 2])
    }

    #[test]
    fn get_pairs_of_2_works() {
        let h = PokerHand::from_str("2H 2H 7C 3C 7D");
        assert_eq!(h.get_pairs_of_n_helper(2), hashset!(2, 7));
    }

    #[test]
    fn get_pairs_of_n_is_empty_when_no_pairs() {
        let h = PokerHand::from_str("1H 2H 4C 3C 7D");
        assert_eq!(h.get_pairs_of_n_helper(2), hashset!())
    }

    #[test]
    fn get_paris_of_3_works() {
        let h = PokerHand::from_str("2H 2D 2C 3C 7D");
        assert_eq!(h.get_pairs_of_n_helper(3), hashset!(2))
    }

    #[test]
    fn get_paris_of_4_works() {
        let h = PokerHand::from_str("2H 2D 2C 2S 7D");
        assert_eq!(h.get_pairs_of_n_helper(4), hashset!(2))
    }

    #[test]
    fn score_equality_works() {
        //equal
        assert_eq!(Score::FiveOfAKind, Score::FiveOfAKind);
        assert_eq!(Score::StraightFlush(10), Score::StraightFlush(10));
        assert_eq!(Score::FourOfAKind(3, 2), Score::FourOfAKind(3, 2));
        assert_eq!(Score::ThreeOfAKind(3, 1, 2), Score::ThreeOfAKind(3, 1, 2));
        assert_eq!(Score::ThreeOfAKind(3, 1, 2), Score::ThreeOfAKind(3, 2, 1));
        assert_eq!(Score::TwoPair(2, 5, 8), Score::TwoPair(2, 5, 8));
        assert_eq!(Score::TwoPair(5, 2, 8), Score::TwoPair(2, 5, 8));
        assert_eq!(Score::OnePair(6), Score::OnePair(6));
        assert_eq!(
            Score::HighCard(vec![6, 5, 4, 3, 2]),
            Score::HighCard(vec![6, 5, 4, 3, 2])
        );
        assert_eq!(Score::FullHouse(5, 8), Score::FullHouse(5, 8));
        assert_eq!(
            Score::Flush(vec![9, 8, 7, 6, 5]),
            Score::Flush(vec![9, 8, 7, 6, 5])
        );
        assert_eq!(Score::Straight(9), Score::Straight(9));

        //not equal
        assert_ne!(Score::StraightFlush(10), Score::StraightFlush(9));
        assert_ne!(Score::FourOfAKind(4, 4), Score::FourOfAKind(3, 4));
        assert_ne!(Score::FourOfAKind(4, 2), Score::FourOfAKind(4, 3));
        assert_ne!(Score::ThreeOfAKind(4, 3, 4), Score::ThreeOfAKind(3, 3, 4));
        assert_ne!(Score::ThreeOfAKind(4, 3, 1), Score::ThreeOfAKind(4, 3, 2));
        assert_ne!(Score::ThreeOfAKind(4, 1, 2), Score::ThreeOfAKind(4, 3, 1));
        assert_ne!(Score::TwoPair(5, 2, 7), Score::TwoPair(2, 8, 9));
        assert_ne!(Score::FullHouse(8, 5), Score::FullHouse(5, 8));
    }

    #[test]
    fn score_partial_cmp_works() {
        assert!(Score::FiveOfAKind > Score::FourOfAKind(8, 10));
        assert!(Score::StraightFlush(10) > Score::FourOfAKind(8, 10));
        assert!(Score::FourOfAKind(8, 10) > Score::FullHouse(8, 5));
        assert!(Score::FullHouse(8, 5) > Score::Flush(vec![5, 4, 3, 2, 1]));
        assert!(Score::Flush(vec![5, 4, 3, 2, 1]) > Score::Straight(5));
        assert!(Score::Straight(5) > Score::ThreeOfAKind(5, 2, 1));
        assert!(Score::ThreeOfAKind(5, 2, 1) > Score::TwoPair(5, 2, 3));
        assert!(Score::TwoPair(5, 2, 3) > Score::OnePair(10));
        assert!(Score::OnePair(10) > Score::HighCard(vec![10, 7, 6, 5, 4]));

        assert!(Score::StraightFlush(10) > Score::StraightFlush(9));
        assert!(Score::FourOfAKind(8, 10) > Score::FourOfAKind(8, 5));
        assert!(Score::Flush(vec![7, 6, 5, 4, 3]) > Score::Flush(vec![7, 5, 4, 3, 2]));
        assert!(Score::FullHouse(8, 4) > Score::FullHouse(4, 8));
        assert!(Score::FullHouse(8, 5) > Score::FullHouse(8, 4));
        assert!(Score::HighCard(vec![8, 6, 4, 3, 2]) > Score::HighCard(vec![8, 5, 4, 3, 2]));
        assert!(Score::TwoPair(5, 2, 8) > Score::TwoPair(5, 2, 4));
        assert!(Score::ThreeOfAKind(5, 3, 1) > Score::TwoPair(5, 2, 1));
        assert!(Score::ThreeOfAKind(5, 3, 2) > Score::TwoPair(5, 3, 1));
    }
}
