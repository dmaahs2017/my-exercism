use crate::card::Rank;
use std::cmp;

type Kicker = u8;

#[derive(Debug)]
pub enum Score {
    FiveOfAKind,
    StraightFlush(Rank),
    FourOfAKind(Rank, Kicker),
    FullHouse(Rank, Kicker),
    Flush(Vec<Rank>),
    Straight(Rank),
    ThreeOfAKind(Rank, Kicker, Kicker),
    TwoPair(Rank, Rank, Kicker),
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

#[cfg(test)]
mod tests {
    use super::*;

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
