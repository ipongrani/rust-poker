use crate::card::Card;

#[derive(Debug, Clone)]
pub struct HandRank {
    pub hand_rank: u8,
    pub hand_rank_name: String,
    pub winning_cards: Vec<Card>,
    pub high_card: Card,
    pub kicker: Card,
}

impl PartialEq for HandRank {
    fn eq(&self, other: &Self) -> bool {
        // Compare the fields that determine equality
        self.hand_rank == other.hand_rank &&
        self.hand_rank_name == other.hand_rank_name &&
        self.winning_cards == other.winning_cards &&
        self.high_card == other.high_card
    }
}