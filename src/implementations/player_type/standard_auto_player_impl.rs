use crate::features::{StandardAutoPlayer, HandRank, ActionResult};
use crate::constants::PlayerAction;
use crate::card::Card;
use crate::Player;
use rand::Rng;



impl StandardAutoPlayer for Player {
    fn deduct_funds(&mut self, purpose: String, amount: u32) -> u32 {
        let current_coins: u32 = self.get_coins().clone();

        let mut proceed_deduction = |amount_to_deduct: u32| -> u32 {
            if amount_to_deduct <= current_coins {
                if let Some(result) = current_coins.checked_sub(amount) {
                    self.set_coins(result);
                    amount
                } else {
                    println!("Error move. coins left: {:?}", current_coins);
                    0
                }
            } else {
                println!("Not enough coins for this move");
                0
            }
        };

        match purpose.as_str() {
            "Blinds" => proceed_deduction(amount),
            "Bet" => {
                let amount_increase_for_bet = proceed_deduction(amount);
                self.increase_bet(amount_increase_for_bet);
                amount_increase_for_bet
            },
            "Raise" => {
                let amount_increase_for_bet = proceed_deduction(amount);
                self.increase_bet(amount_increase_for_bet);
                amount_increase_for_bet
            },
            "Call" => {
                let amount_increase_for_bet = proceed_deduction(amount);
                self.increase_bet(amount_increase_for_bet);
                amount_increase_for_bet
            },
            _ => {
                println!("Purpose not authorized to take funds");
                0
            }
        }
    }
    

    fn get_current_bet(&self) -> u32 {
        self.get_current_bet()
    }

    fn has_player_folded(&self) -> bool {
        self.get_folded()
    }

    fn set_handrank(&mut self, handrank: HandRank) {
        self.hand_rank = Some(handrank);
    }

    fn get_handrank(&mut self, handrank: HandRank) {
        self.hand_rank = Some(handrank);
    }

    fn get_bet_amount(&self, minimum_bet: u32) -> u32 {
        // Implement your logic here to determine the bet amount based on the player's hand and game rules
        // Return the bet amount
    
        // Temporary random logic for bet amount
        
        if minimum_bet > self.get_coins() {
            self.get_coins()
        } else {
            let min_bet = 1;
            let max_bet = self.get_coins();
            let bet_amount = rand::thread_rng().gen_range(min_bet..=max_bet);
            if bet_amount > self.get_coins() {
                self.get_coins()
            } else {
                bet_amount
            }
        }
        
    }

    fn get_raise_amount(&self) -> u32 {
        // Implement your logic here to determine the bet amount based on the player's hand and game rules
        // Return the bet amount
    
        // Temporary random logic for bet amount
        let min_bet = 1;
        let max_bet = std::cmp::min(self.get_coins(), 100);
        let bet_amount = rand::thread_rng().gen_range(min_bet..=max_bet);
    
        if bet_amount > self.get_coins() {
            self.get_coins()
        } else {
            bet_amount
        }
    }

    fn fold(&mut self) {
        self.set_folded(true);
    }

    fn receive_card(&mut self, card: Option<Card>) {
        match self.get_current_hand() {
            hand_cards => {
                if hand_cards.len() < 2 {
                   if let Some(card) = card {
                        hand_cards.push(card);
                   } else {
                        println!("Player; {} did not receive a card.", self.get_player_id())
                   }
                } else {
                    println!("Hand card is full.");
                }
            }
         }
    }

    fn get_hand_length(&mut self) -> usize {
        let hand_cards = self.get_current_hand();
        hand_cards.len()
    }

    fn request_funds(&mut self, amount: u32, purpose: String, requester: &str) -> u32 {
        if requester.contains("dlr") {
            self.deduct_funds(purpose, amount)
        } else {
            println!("Not authorized to request funds!");
            0
        }
    }

    fn request_hand_cards(&mut self, requester: &str) -> Option<&Vec<Card>> {
        if requester.contains("dlr") {
            Some(self.get_current_hand())
        } else {
            println!("Not authorized to request cards!");
            None
        }
    }

    fn get_action(&self, minimum_bet: Option<u32>) -> ActionResult {
        // Determine the valid actions based on available chips and community cards
        let mut valid_actions: Vec<PlayerAction> = Vec::new();
    
        if self.has_player_folded() {
            // Player has folded, so they can't take any action
            return ActionResult { action: PlayerAction::Fold, success: false, amount: None };
        }
    
        if self.get_coins() == 0 {
            // Player has no chips left, so they can only check or fold
            valid_actions.push(PlayerAction::Check);
            valid_actions.push(PlayerAction::Fold);
        } else {
            // Player has chips remaining, so they can choose from all actions
            valid_actions.push(PlayerAction::Check);
            valid_actions.push(PlayerAction::Bet);
            valid_actions.push(PlayerAction::Raise);
            valid_actions.push(PlayerAction::Call);
            valid_actions.push(PlayerAction::Fold);
        }
    
        // Implement your logic for choosing an action based on the valid actions
        // For example, you can use random selection, AI strategy, or user input
    
        // Randomly choose an action from the valid actions
        let random_action = rand::thread_rng().gen_range(0..valid_actions.len());
        let chosen_action = &valid_actions[random_action];
    
        // Return the chosen action with a success flag
        match chosen_action {
            PlayerAction::Bet => ActionResult { action: PlayerAction::Bet, success: true, amount: Some(self.get_bet_amount(minimum_bet.unwrap_or(0))) },
            PlayerAction::Raise => ActionResult { action: PlayerAction::Raise, success: true, amount: Some(self.get_raise_amount()) },
            PlayerAction::Call => ActionResult { action: PlayerAction::Call, success: true, amount: None },
            _ => ActionResult { action: chosen_action.clone(), success: true, amount: None },
        }
    }
}

