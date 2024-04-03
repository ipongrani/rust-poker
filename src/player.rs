
use super::card::Card;
use crate::features::{HandRank, ActionResult, StandardAutoPlayer};
use crate::constants::PlayerVersion;

#[derive(Debug, Clone)]
pub struct Player {
    player_type: PlayerVersion,
    id: u32,
    username: String,
    coins: u32,
    folded: bool,
    hand_cards: Vec<Card>,
    current_bet: u32,
    pub hand_rank: Option<HandRank>
}



impl Player {
    pub fn new(player_type: &str, id: u32, username: String, coins: u32,) -> Self {
        let converted_player_type:PlayerVersion = match player_type {
            "AutoPlayer" => PlayerVersion::AutoPlayer,
            _ => PlayerVersion::AutoPlayer
        };
        Player {
            player_type: converted_player_type,
            id,
            username,
            coins,
            folded: false,
            hand_cards: Vec::new(),
            current_bet: 0,
            hand_rank: None
        }
    }

    pub fn get_player_id(&self) -> u32 {
        self.id
    }

    pub fn get_player_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_coins(&self) -> u32 {
        self.coins
    }

    pub fn set_coins(&mut self, coins: u32) {
        self.coins = coins;
    }

    pub fn increase_bet(&mut self, amount: u32) {
        self.current_bet = self.current_bet + amount;
    }

    pub fn get_current_bet(&self) -> u32 {
        self.current_bet.clone()
    }

    pub fn get_folded(&self) -> bool {
        self.folded
    }

    pub fn set_folded(&mut self, fold: bool) {
        self.folded = fold;
    }

    pub fn get_current_hand(&mut self) -> &mut Vec<Card> {
        &mut self.hand_cards
    }

    pub fn get_action(&self, minimum_bet: u32) -> ActionResult {
        match self.player_type {
            PlayerVersion::AutoPlayer => StandardAutoPlayer::get_action(self, Some(minimum_bet))
        }
    }

}


impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        // Compare the fields that determine equality
        self.id == other.id &&
        self.username == other.username &&
        self.coins == other.coins &&
        self.folded == other.folded &&
        self.hand_cards.iter().all(|card| other.hand_cards.contains(card)) &&
        self.current_bet == other.current_bet &&
        self.hand_rank == other.hand_rank
    }
}