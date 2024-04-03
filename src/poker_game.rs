
use super::dealer::Dealer;
use super::player::{Player};
use super::card::Card;
use crate::constants::{PlayerAction};
use crate::features::StandardAutoPlayer;
use crate::features::PokerRules;
use rand::Rng;


pub struct PokerGame {
    game_id: String,
    players: Vec<Player>,
    game_dealer: Dealer,
    pot: u32,
    with_blinds: Option<bool>,
    small_blind_index: usize,
    big_blind_index: usize,
    blinds_initialized: bool,
    big_blind: u32,
    game_state: String,
    highest_bet: u32,
    community_deck: Vec<Card>
}


impl PokerGame {
    pub fn new(players: Vec<Player>, game_dealer: Dealer) -> Self {
        let game_id = String::from("pkrgme-")+&Self::generate_game_id(9);
        let small_blind_index = rand::thread_rng().gen_range(0..players.len());
        let big_blind_index = (small_blind_index + 1) % players.len();
        PokerGame {
            game_id,
            players,
            game_dealer,
            pot: 0,
            with_blinds: None,
            small_blind_index,
            big_blind_index,
            blinds_initialized: false,
            big_blind: 0,
            game_state: String::from("initial"),
            highest_bet: 0,
            community_deck: Vec::new()
        }
    }
    
    fn generate_game_id(id_length: usize) -> String {
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

    fn are_players_ready(&self) -> bool {
        let players_with_cards:&Vec<Player> = &self.players.iter().cloned().filter(|player| player.clone().get_hand_length() > 0).collect();
        if self.players.len() == players_with_cards.len() {
            true
        } else {
            false
        }
    }

    fn is_dealer_ready(&self) -> bool {
        let deck_id = self.game_dealer.get_deck_id();
        match deck_id {
            "" => {
                println!("Dealer has no deck yet. Generate a deck first.");
                return false;
            }
            _ => {
                let mut is_ready = false;
                if let Some(with_blinds) = self.with_blinds {
                    match with_blinds {
                        true => {
                            if self.blinds_initialized == false {
                                println!("Please set blind amount if playing with blinds.");
                                is_ready = false;
                            } else {
                                is_ready = true;
                            }
                        },
                        false => {
                            is_ready = true;
                        },
                    }
                } else {
                    println!("Please specify if playing with blinds.");
                }
                is_ready
            }
        }
    }
    
    fn add_to_pot(&mut self, amount: u32) {
        self.pot = self.pot + amount;
    }

    fn set_game_state(&mut self, new_game_state: &str) {
        self.game_state = String::from(new_game_state);
    }
    
    fn check_round_over(&self) -> bool {
        let remaining_players: Vec<&Player> = self.players.iter()
            .filter(|player| !player.has_player_folded())
            .collect();
        
        let all_bets_equal = remaining_players.iter().all(|player| player.get_current_bet() == self.highest_bet);
        
        remaining_players.len() == 1 || all_bets_equal
    }

    fn bet(&mut self, player_index: usize, bet_amount: u32) -> bool {
        let player = &mut self.players[player_index];
        let min_bet = self.big_blind;
        let mut bet_successful = false;
                
        if bet_amount >= min_bet {
            let requested_funds = self.game_dealer.player_bet(player, bet_amount);
            if requested_funds > 0 {
                self.highest_bet = self.highest_bet + requested_funds;
                self.add_to_pot(requested_funds);
                bet_successful = true;
            }
        } else {
            println!("Minimum bet is: {}", min_bet);
        }
        bet_successful
    }

    fn raise(&mut self, player_index: usize, raise_amount: u32) -> bool {
        let player = &mut self.players[player_index];
        let min_raise = self.highest_bet*2;        
        let mut raise_successful = false;

        if raise_amount > min_raise {
            let requested_funds = self.game_dealer.player_raise(player, raise_amount);
            if requested_funds > 0 {
                self.highest_bet = self.highest_bet + requested_funds;
                self.add_to_pot(requested_funds);
                raise_successful = true;
            }
        } else {
            println!("Minimum raise is: {}", min_raise);
        }
        raise_successful
      }

    fn call(&mut self, player_index: usize) -> bool {
        let player = &mut self.players[player_index];
        let current_player_bet =  player.get_current_bet().clone();
        let current_player_coins = player.get_coins().clone();
        let amount_to_call = self.highest_bet - current_player_bet;   
        let mut call_successful = false;

        if amount_to_call <= current_player_coins {
            let requested_funds = self.game_dealer.player_call(player, amount_to_call);
            if requested_funds > 0 {
                self.highest_bet = self.highest_bet + requested_funds;
                self.add_to_pot(requested_funds);
                call_successful = true;
            }
        } else {
            println!("Funds required to call");
        }
        call_successful
    }


    fn fold(&mut self, player_index: usize) -> bool {
        let remaining_players:&Vec<Player> = &self.players.iter()
            .filter(|player| !player.has_player_folded())
            .cloned()
            .collect();
        let has_everyone_else_folded:bool = remaining_players.len() == 1;
        if !has_everyone_else_folded {
            let player = &mut self.players[player_index];
            player.fold();
            true
        } else {
            false
        }
    }
    
    fn check(&mut self, player_index: usize) -> bool {
        println!("player index: {}", player_index);
        true
    }

    fn round_decisions(&mut self, current_player_index: Option<usize>) -> bool {
        println!("highest bet: {:?}", self.highest_bet);
    
        let player_index = current_player_index.unwrap_or(0);
        let mut next_player_index = (player_index + 1) % self.players.len();
        let player = &mut self.players[player_index];
    
        if !player.has_player_folded() {
            let player_decision = player.get_action(self.big_blind.clone());
            let action = player_decision.action;
            let action_success = player_decision.success;
            let amount = player_decision.amount.unwrap_or_default();
    
            match action_success {
                true => {
                    match action {
                        PlayerAction::Bet => {
                            println!("{:?} chose to bet {}", player.get_player_username(), amount);
                            let player_action_successful = self.bet(player_index, amount);
                            if !player_action_successful {
                                println!("Invalid bet amount. Try a different action.");
                                return false;
                            }
                        },
                        PlayerAction::Raise => {
                            println!("{:?} chose to raise {}", player.get_player_username(), amount);
                            let player_action_successful = self.raise(player_index, amount);
                            if !player_action_successful {
                                println!("Invalid raise amount. Try a different action.");
                                return false;
                            }
                        },
                        PlayerAction::Call => {
                            println!("{:?} chose to call", player.get_player_username());
                            let player_action_successful = self.call(player_index);
                            if !player_action_successful {
                                println!("Insufficient funds to call. Try a different action.");
                                return false;
                            }
                        },
                        PlayerAction::Check => {
                            println!("{:?} chose to check", player.get_player_username());
                            let player_action_successful = self.check(player_index);
                            if !player_action_successful {
                                println!("Invalid check. Try a different action.");
                                return false;
                            }
                        },
                        PlayerAction::Fold => {
                            println!("{:?} chose to fold", player.get_player_username());
                            let player_action_successful = self.fold(player_index);
                            if !player_action_successful {
                                println!("Failed to fold. Try a different action.");
                                return false;
                            }
                        }
                    }
                },
                false => {
                    println!("Failed to get action from player");
                    return false;
                }
            }
        } else {
            next_player_index = (next_player_index + 1) % self.players.len();
        }
    
        println!("*************** is round over: {:?} ***************", self.check_round_over());
        if !self.check_round_over() {
            return self.round_decisions(Some(next_player_index));
        } else {
            println!("===================================== Round Over ============================================");
            return true;
        }
    }

    fn request_dealer_open_cards(&mut self, card_amount_to_open: usize) {
        if let Some(retrieved_cards) = self.game_dealer.open_cards(card_amount_to_open) {
            self.community_deck.extend(retrieved_cards);
        }
    }

    fn show_hands(&self) -> Vec<Player> {
        let filter_check:usize = 0 as usize;
        let eligible_players:Vec<Player> = self.players.iter()
                                                            .cloned()
                                                            .filter(|player| player.clone().get_hand_length() > filter_check && !player.has_player_folded())
                                                            .map(|player| self.get_hand_rank(player, &self.game_dealer, &self.community_deck))
                                                            .collect();

        eligible_players
    }


    fn determine_winner(&self) {
        let players = self.show_hands();
        let result = players.iter().fold(
            (None, false, Vec::new()),
            |(winner, tie_winners, mut tie_players), player| {
                if winner.is_none() {
                    (Some(player), false, Vec::new())
                } else if player.hand_rank.as_ref().unwrap().hand_rank > winner.as_ref().unwrap().hand_rank.as_ref().unwrap().hand_rank {
                    (Some(player), false, Vec::new())
                } else if player.hand_rank.as_ref().unwrap().hand_rank == winner.as_ref().unwrap().hand_rank.as_ref().unwrap().hand_rank {
                    let player_rank = player.hand_rank.as_ref().unwrap().high_card.get_rank().parse::<u32>();
                    let winner_rank = winner.as_ref().unwrap().hand_rank.as_ref().unwrap().high_card.get_rank().parse::<u32>();
        
                    if player_rank.is_err() || winner_rank.is_err() {
                        (winner, tie_winners, tie_players)
                    } else {
                        let player_rank = player_rank.unwrap();
                        let winner_rank = winner_rank.unwrap();
        
                        if player_rank > winner_rank {
                            (Some(player), false, Vec::new())
                        } else if player_rank == winner_rank {
                            if !tie_players.contains(winner.as_ref().unwrap()) {
                                tie_players.push(winner.as_ref().unwrap().clone());
                            }
                            tie_players.push(player);
                            (winner, true, tie_players)
                        } else {
                            (winner, tie_winners, tie_players)
                        }
                    }
                } else {
                    (winner, tie_winners, tie_players)
                }
            },
        );
    
        let cloned_result = result.clone();
        //let final_winner = cloned_result.0;
        let mut tie_winners = cloned_result.1;
        let mut tie_players = cloned_result.2;
    
        // Handle final winner and tie scenarios
        if tie_players.len() == 1 {
            let winner = tie_players[0];
            tie_winners = false;
            tie_players = Vec::new();
            println!("Winner: {:?}", winner);
        } else {
            println!("result: {:?}", result);
        }

        println!("tie_winners: {:?}", tie_winners);
        println!("tie_players: {:?}", tie_players);
    
        // Perform any additional actions with the winner, tie players, etc.
    }




    pub fn play_with_blinds(&mut self, with_blinds: bool) {
        self.with_blinds = Some(with_blinds);
    }

    pub fn show_status(&self) {
        let dealer_id = self.game_dealer.get_dealer_id();
        let dealer_type = self.game_dealer.get_dealer_type();
        let deck_id = self.game_dealer.get_deck_id();
        let current_players = &self.players;
        let current_dealer = &self.game_dealer;
        let game_id = &self.game_id;
        let pot = &self.pot;
        let community_deck = &self.community_deck;
        
        println!("current_players");
        for player in current_players {
            println!("player: {:?}", player);
        }
        //println!("current_dealer: {:?}", current_dealer);
        println!("dealer_id: {}", dealer_id);
        println!("dealer_type: {}", dealer_type);
        //println!("dealer deck: {:?}", current_dealer.get_deck());
        println!("dealer deck length: {:?}", current_dealer.get_deck().as_ref().map(|deck| deck.len()).unwrap_or(0));
        println!("deck_id: {}", deck_id);
        println!("game_id: {}", game_id);
        println!("pot: {}", pot);
        println!("community_deck: {:?}", community_deck);
    }

    pub fn request_generate_deck(&mut self, deck_count: usize) {
        self.game_dealer.generate_cards(deck_count);
    }

    pub fn request_dealer_shuffle(&mut self) {
        self.game_dealer.shuffle_cards();
    }

    pub fn deal_cards(&mut self) {
        if self.is_dealer_ready() {
            let current_players = &mut self.players;
            current_players.iter_mut().for_each(|player| self.game_dealer.deal_player(&mut Some(player)));
        } else {
            println!("Unable to deal cards. Dealer is not ready.");
        }
    }

    pub fn post_blinds_current_game(&mut self, amount: u32) {
        println!("amount: {}", amount);
        if let Some(with_blinds) = self.with_blinds {
            if self.blinds_initialized == false && with_blinds == true {
                let small_blind:u32 = amount;
                let big_blind:u32 = amount*2;
                self.big_blind = big_blind.clone();

                {
                    let small_blind_player = &mut self.players[self.small_blind_index];
                    let requested_fund = self.game_dealer.post_blinds(small_blind_player, small_blind);
                    self.add_to_pot(requested_fund);
                }
        
                {
                    let big_blind_player:&mut Player = &mut self.players[self.big_blind_index];
                    let requested_fund = self.game_dealer.post_blinds(big_blind_player, big_blind);
                    self.add_to_pot(requested_fund);
                    self.highest_bet = requested_fund;
                }
    
                self.blinds_initialized = true;
            } else {
                println!("Blinds has been set already set for this game.");
            }
        } else {
            println!("Blinds not enabled for this game");
        }
    }

    

    
    pub fn play(&mut self) {
        let game_id:&str = &self.game_id;
        match game_id {
            "" => {
                // game_id is empty
                println!("You need to create a game first or load an existing game");
                return;
            }
            _ => {
                // game_id has a value
                let dealer_ready:bool = self.is_dealer_ready();
                println!("Dealer is ready: {}", dealer_ready);
                if dealer_ready {
                    match self.game_state.as_str() {
                        "initial" => {
                            println!("Game at initial state.");
                            match self.are_players_ready() {
                                false => {
                                    println!("Please deal players with cards.")
                                },
                                _ => {
                                    self.set_game_state("Pre-flop");
                                    self.play();
                                }
                            }
                        },
                        "Pre-flop" => {
                            println!("Pre-flop");
                            let round_ended = self.round_decisions(None);
                            println!("pre-flop round ended: {}", round_ended);
                            if round_ended {
                                self.set_game_state("Flop");
                            }
                            self.play();
                        },
                        "Flop" => {
                            println!("Flop");
                            self.request_dealer_open_cards(3);
                            let round_ended = self.round_decisions(None);
                            if round_ended {
                                self.set_game_state("Turn");
                            }
                            self.play();
                        },
                        "Turn" => {
                            println!("Turn");
                            self.request_dealer_open_cards(1);
                            let round_ended = self.round_decisions(None);
                            if round_ended {
                                self.set_game_state("River");
                            }
                            self.play();
                        },
                        "River" => {
                            println!("River");
                            self.request_dealer_open_cards(1);
                            let round_ended = self.round_decisions(None);
                            if round_ended {
                                self.set_game_state("ShowHands");
                            }
                            self.play();
                        },
                        "ShowHands" => {
                            println!("ShowHands");
                            self.determine_winner();
                            //self.round_decisions(None);
                            //self.set_game_state("River");
                            //self.play();
                        },
                        _ => {
                            println!("Game state invalid!");
                        }
                    }
                }
            }
        }
    }
}