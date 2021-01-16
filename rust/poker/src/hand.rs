use crate::card::{Card, Rank};
use crate::score::Score;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Hand<'a> {
    hand: Vec<Card>,
    str_hand: &'a str,
}

impl<'a> Hand<'a> {
    pub fn from_str(str_hand: &'a str) -> Self {
        Hand {
            hand: str_hand
                .split_whitespace()
                .map(|c| Card::from_str(c))
                .collect(),
            str_hand,
        }
    }

    pub fn get_str(&self) -> &'a str {
        self.str_hand
    }

    pub fn score_hand(&self) -> Score {
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
    fn poker_hand_get_str_works() {
        let h = "2H 2H 2H 2H 2H";
        let ph = Hand::from_str(h);
        assert_eq!(ph.get_str(), h)
    }

    #[test]
    fn poker_hand_from_str_works() {
        let h = "2H 3D 10S JC KS";
        let ph = Hand::from_str("2H 3D 10S JC KS");
        assert!(ph.is_same_hand(&Hand {
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
    fn get_ranks_sorted_high_to_low_works() {
        let h = Hand::from_str("2H KH 10C JD KC");
        assert_eq!(h.get_ranks_sorted_high_to_low(), vec![13, 13, 11, 10, 2])
    }

    #[test]
    fn get_pairs_of_2_works() {
        let h = Hand::from_str("2H 2H 7C 3C 7D");
        assert_eq!(h.get_pairs_of_n_helper(2), hashset!(2, 7));
    }

    #[test]
    fn get_pairs_of_n_is_empty_when_no_pairs() {
        let h = Hand::from_str("1H 2H 4C 3C 7D");
        assert_eq!(h.get_pairs_of_n_helper(2), hashset!())
    }

    #[test]
    fn get_paris_of_3_works() {
        let h = Hand::from_str("2H 2D 2C 3C 7D");
        assert_eq!(h.get_pairs_of_n_helper(3), hashset!(2))
    }

    #[test]
    fn get_paris_of_4_works() {
        let h = Hand::from_str("2H 2D 2C 2S 7D");
        assert_eq!(h.get_pairs_of_n_helper(4), hashset!(2))
    }
}
