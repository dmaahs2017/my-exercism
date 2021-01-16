use crate::card::Rank;

type Kicker = u8;

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
pub enum Score {
    HighCard(Vec<Rank>), // must be sorted high to low
    OnePair(Rank),
    TwoPair(Rank, Rank, Kicker), // ranks must be ordered high to low
    ThreeOfAKind(Rank, Kicker, Kicker), //kickers must be sorted high to low
    Straight(Rank),
    Flush(Vec<Rank>), // must be sorted high to low
    FullHouse(Rank, Kicker),
    FourOfAKind(Rank, Kicker),
    StraightFlush(Rank),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_cmp_is_lexicographical() {
        assert!(Score::TwoPair(10, 5, 3) > Score::OnePair(13));
        assert!(Score::TwoPair(10, 5, 4) > Score::TwoPair(10, 5, 3));
        assert!(Score::TwoPair(10, 6, 4) > Score::TwoPair(10, 5, 4));
        assert!(Score::TwoPair(13, 6, 4) > Score::TwoPair(10, 6, 4));
    }

    #[test]
    fn vec_cmp_is_lexicographical() {
        assert!(Score::Flush(vec![10, 8, 7, 5, 2]) > Score::Flush(vec![9, 8, 7, 5, 2]));
        assert!(Score::Flush(vec![10, 8, 7, 5, 2]) > Score::Flush(vec![10, 8, 7, 5, 1]));
    }

    #[test]
    fn three_of_a_kind_works() {
        assert!(Score::ThreeOfAKind(13, 7, 4) > Score::ThreeOfAKind(13, 6, 4));
        assert!(Score::ThreeOfAKind(13, 6, 5) > Score::ThreeOfAKind(13, 6, 4));
    }
}
