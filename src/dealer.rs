
use super::card::Card;
use super::player::Player;
use crate::features::StandardAutoPlayer;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::rngs::OsRng;


#[derive(Debug, Clone)]
pub struct Dealer {
    dealer_id: String,
    dealer_type: String,
    deck_id: Option<String>,
    deck: Option<Vec<Card>>,
    //assigned_game: Option<String>
}


impl Dealer {
    pub fn new(dealer_type: String) -> Self {
        let dealer_id:String = String::from("dlr-")+&Self::generate_id(9);
        Dealer {
            dealer_id,
            dealer_type,
            deck_id: None,
            deck: None,
            //assigned_game: None
        }
    }

    fn generate_id(id_length: usize) -> String {
        let mut rng = rand::thread_rng();
        let id_characters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();
        
        let new_game_id: String = (0..id_length)
            .map(|_| {
                let idx = rng.gen_range(0..id_characters.len());
                id_characters[idx]
            })
            .collect();
        
        new_game_id
    }

    fn get_a_card(&mut self) -> Option<Card> {
        match &mut self.deck {
            Some(deck) => {
                if deck.len() > 0 {
                    let newly_taken_card = deck.pop();
                    newly_taken_card
                } else {
                    println!("Deck is Empty!");
                    None
                }
            },
            _ => {
                println!("Unable to get a card");
                None
            }
        }
    }

    pub fn post_blinds(&self, player: &mut Player, amount: u32) -> u32 {
        player.request_funds(amount, String::from("Blinds"), &self.get_dealer_id())
    }

    pub fn player_bet(&self, player: &mut Player, amount: u32) -> u32 {
        player.request_funds(amount, String::from("Bet"), &self.get_dealer_id())
    }

    pub fn player_raise(&self, player: &mut Player, amount: u32) -> u32 {
        player.request_funds(amount, String::from("Raise"), &self.get_dealer_id())
    }

    pub fn player_call(&self, player: &mut Player, amount: u32) -> u32 {
        player.request_funds(amount, String::from("Raise"), &self.get_dealer_id())
    }

    pub fn get_dealer_id(&self) -> &str {
        let dealer_id:&str = &self.dealer_id;
        dealer_id
    }

    pub fn get_dealer_type(&self) -> &str {
        let dealer_type:&str = &self.dealer_type;
        dealer_type
    }

    pub fn get_deck_id(&self) -> &str{
        let deck_id:&str = &self.deck_id.as_ref()
            .map(String::as_str).unwrap_or_default();
        deck_id
    }

    pub fn generate_cards(&mut self, num_decks: usize) -> bool{
        let current_deck:Vec<Card> = match &self.deck {
            Some(deck) => deck.iter().cloned().collect(),
            None => Vec::new(),
        };
        match current_deck.len() {
            0 => {
                let suits = ["Spades", "Hearts", "Diamonds", "Clubs"];
                let ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];
            
                fn generate_cards_recursive(num_decks: usize, suits: &[&str], ranks: &[&str]) -> Vec<Card> {
                    fn generate_recursive(
                        cards: &mut Vec<Card>,
                        num_decks: usize,
                        suits: &[&str],
                        ranks: &[&str],
                        deck: usize,
                        suit: usize,
                        rank: usize,
                    ) {
                        if deck < num_decks {
                            if suit < suits.len() {
                                if rank < ranks.len() {
                                    cards.push(Card::new(String::from(ranks[rank]), String::from(suits[suit])));
                                    generate_recursive(cards, num_decks, suits, ranks, deck, suit, rank + 1);
                                } else {
                                    generate_recursive(cards, num_decks, suits, ranks, deck, suit + 1, 0);
                                }
                            } else {
                                generate_recursive(cards, num_decks, suits, ranks, deck + 1, 0, 0);
                            }
                        }
                    }
            
                    let mut cards: Vec<Card> = Vec::new();
                    generate_recursive(&mut cards, num_decks, suits, ranks, 0, 0, 0);
                    cards
                }
            
                let deck_id:String = String::from("deck-")+&Self::generate_id(9);
                let cards = generate_cards_recursive(num_decks, &suits, &ranks);
                self.deck_id = Some(deck_id);
                self.deck = Some(cards);
                true
            }
            _ => false
        }

    }

    pub fn get_deck(&self) -> &Option<Vec<Card>> {
        let current_deck:&Option<Vec<Card>> = &self.deck;
        current_deck
    }

    pub fn shuffle_cards(&mut self) {
        let mut rng = OsRng::default();
        match &mut self.deck {
            Some(deck) => {
                deck.shuffle(&mut rng);
            },
            _ => {
                println!("Dealer has no Deck to shuffle!");
            }
        };
    }
    
    pub fn deal_player(&mut self, player: &mut Option<&mut Player>) {
        if let Some(_deck) = &self.deck {
            if let Some(player) = player {
                player.receive_card(self.get_a_card());
                player.receive_card(self.get_a_card());
            } else {
                println!("No players to deal!");
            }
        } else {
            println!("Dealer has no deck to deal");
        }
        
    }

    pub fn open_cards(&mut self, number_of_cards: usize) -> Option<Vec<Card>> {
        if let Some(_deck) = &self.deck {
           let mut cards_for_community = Vec::new();
            for _idx in 0..number_of_cards {
                cards_for_community.push(self.get_a_card().unwrap());
            }
            Some(cards_for_community)
        } else {
            println!("Dealer has no deck to deal");
            None
        }
        
    }

    pub fn request_player_hand(&mut self, player: &Player) -> Option<Vec<Card>> {
        if let Some(retrieved_hand_cards) = &player.clone().request_hand_cards(self.get_dealer_id()).cloned() {
            Some(retrieved_hand_cards.clone())
        } else {
            None
        }
    }

}