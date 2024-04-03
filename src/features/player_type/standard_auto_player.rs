use crate::card::Card;
use crate::features::{ActionResult, HandRank};



pub trait StandardAutoPlayer {
    fn deduct_funds(&mut self, purpose: String, amount: u32) -> u32;
    fn get_current_bet(&self) -> u32;
    fn has_player_folded(&self) -> bool;
    fn set_handrank(&mut self, handrank: HandRank);
    fn get_handrank(&mut self, handrank: HandRank);
    fn get_bet_amount(&self, minimum_bet: u32) -> u32;
    fn get_raise_amount(&self) -> u32;
    fn fold(&mut self);
    fn receive_card(&mut self, card: Option<Card>);
    fn get_hand_length(&mut self) -> usize;
    fn request_funds(&mut self, amount: u32, purpose: String, requester: &str) -> u32;
    fn request_hand_cards(&mut self, requester: &str) -> Option<&Vec<Card>>;
    //fn can_call(&self, highest_bet: u32) -> bool;
    //fn can_bet(&self, minimum_bet: u32) -> bool;
    //fn can_raise(&self) -> bool;
    fn get_action(&self, minimum_bet: Option<u32>) -> ActionResult;
}