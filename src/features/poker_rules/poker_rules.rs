use crate::card::Card;
use crate::player::Player;
use crate::dealer::Dealer;



pub trait PokerRules {
    fn get_flush(hand: &Vec<Card>) -> (bool, Vec<Card>);
    fn get_straight(hand: &Vec<Card>) -> (bool, Vec<Card>);
    fn get_four_of_a_kind(hand: &Vec<Card>) -> (bool, Vec<Card>);
    fn get_full_house(hand: &Vec<Card>) -> (bool, Vec<Card>);
    fn get_three_of_a_kind(hand: &Vec<Card>) -> (bool, Vec<Card>);
    fn get_two_pairs(hand: &Vec<Card>) -> (bool, Vec<Card>);
    fn get_one_pair(hand: &Vec<Card>) -> (bool, Vec<Card>);
    fn get_royal_flush(hand: &Vec<Card>) -> (bool, Vec<Card>);
    fn get_card_rank(card: &Card) -> Option<u32>;
    fn get_kicker(hand: &Vec<Card>, exclude_rank: &str) -> (Option<u32>, Option<Card>);
    fn get_high_card_rank(cards: &Vec<Card>) -> (Option<u32>, Option<Card>);
    fn get_sorted_cards(&self, hand: &mut Vec<Card>) -> Vec<Card>;
    fn get_hand_rank(&self, player: Player, dealer: &Dealer, community_deck: &Vec<Card>) -> Player;
}

