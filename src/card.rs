




#[derive(Debug, Clone)]
pub struct Card {
    rank: String,
    suit: String
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        // Compare the fields that determine equality
        self.rank == other.rank && self.suit == other.suit
    }
}

impl Card {
    pub fn new(rank: String, suit: String) -> Self {
        Card {
            rank,
            suit
        }
    }

    pub fn get_rank(&self) -> &str {
        let rank:&str = &self.rank;
        rank
    }

    pub fn get_suit(&self) -> &str {
        let suit:&str = &self.suit;
        suit
    }
}