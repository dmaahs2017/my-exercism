mod card;
mod hand;
mod score;
use hand::Hand;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    get_best_hands(hands.iter().map(|h| Hand::from_str(h)).collect())
        .map(|hands| hands.iter().map(|h| h.get_str()).collect())
}

fn get_best_hands(mut hands: Vec<Hand>) -> Option<Vec<Hand>> {
    hands.sort_by(|a, b| b.score_hand().cmp(&a.score_hand()));

    if let Some(first) = hands.first() {
        Some(
            hands
                .iter()
                .take_while(|&h| h.score_hand() == first.score_hand())
                .cloned()
                .collect(),
        )
    } else {
        None
    }
}
