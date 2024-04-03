use crate::card::{Card};
use crate::features::{PokerRules, StandardAutoPlayer, HandRank};
use crate::player::Player;
use crate::dealer::Dealer;
use crate::PokerGame;




impl PokerRules for PokerGame {
    fn get_flush(hand: &Vec<Card>) -> (bool, Vec<Card>) {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        let first_suit = sorted_hand[0].get_suit().clone();
        let is_flush = sorted_hand.iter().all(|card| card.get_suit() == first_suit);
        (is_flush, sorted_hand)
    }
    
    fn get_straight(hand: &Vec<Card>) -> (bool, Vec<Card>) {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        let mut is_straight = false;
        if sorted_hand.len() >= 5 {
            let ranks: Vec<u32> = sorted_hand
                .iter()
                .map(|card| Self::get_card_rank(card).unwrap_or_default())
                .collect();
            let mut distinct_ranks: Vec<u32> = ranks.clone();
            distinct_ranks.dedup();
            if distinct_ranks.windows(5).any(|window| window[4].checked_sub(window[0]) == Some(4)) {
                is_straight = true;
            } else if distinct_ranks == vec![2, 3, 4, 5, 14] {
                // Check for A-5 straight
                is_straight = true;
                sorted_hand.retain(|card| {
                    let rank = Self::get_card_rank(card).unwrap_or_default();
                    rank != 14
                });
                sorted_hand.push(Card::new(
                    "1".to_string(),
                    sorted_hand[0].get_suit().to_string(),
                ));
            }
        }
        (is_straight, sorted_hand)
    }

    fn get_four_of_a_kind(hand: &Vec<Card>) -> (bool, Vec<Card>) {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        let ranks: Vec<u32> = sorted_hand
            .iter()
            .map(|card| Self::get_card_rank(card).unwrap_or_default())
            .collect();
        let mut counts: Vec<usize> = vec![0; 15];
        for &rank in ranks.iter() {
            counts[rank as usize] += 1;
        }
        if let Some(rank) = counts.iter().position(|&count| count == 4) {
            let four_of_a_kind_cards: Vec<Card> = sorted_hand
                .iter()
                .filter(|&card| Self::get_card_rank(card) == Some(rank as u32))
                .cloned()
                .collect();
            return (true, four_of_a_kind_cards);
        }
        (false, sorted_hand)
    }

    fn get_full_house(hand: &Vec<Card>) -> (bool, Vec<Card>) {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        let ranks: Vec<u32> = sorted_hand
            .iter()
            .map(|card| Self::get_card_rank(card).unwrap_or_default())
            .collect();
        let mut counts: Vec<usize> = vec![0; 15];
        for &rank in ranks.iter() {
            counts[rank as usize] += 1;
        }
        if let Some(three_rank) = counts.iter().position(|&count| count == 3) {
            if let Some(pair_rank) = counts.iter().position(|&count| count == 2) {
                let three_of_a_kind_cards: Vec<Card> = sorted_hand
                    .iter()
                    .filter(|&card| Self::get_card_rank(card) == Some(three_rank as u32))
                    .cloned()
                    .collect();
                let pair_cards: Vec<Card> = sorted_hand
                    .iter()
                    .filter(|&card| Self::get_card_rank(card) == Some(pair_rank as u32))
                    .cloned()
                    .collect();
                let mut full_house_cards = three_of_a_kind_cards.clone();
                full_house_cards.extend(pair_cards);
                return (true, full_house_cards);
            }
        }
        (false, sorted_hand)
    }
    
    fn get_three_of_a_kind(hand: &Vec<Card>) -> (bool, Vec<Card>) {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        let ranks: Vec<u32> = sorted_hand
            .iter()
            .map(|card| Self::get_card_rank(card).unwrap_or_default())
            .collect();
        let mut counts: Vec<usize> = vec![0; 15];
        for &rank in ranks.iter() {
            counts[rank as usize] += 1;
        }
        if let Some(rank) = counts.iter().position(|&count| count == 3) {
            let three_of_a_kind_cards: Vec<Card> = sorted_hand
                .iter()
                .filter(|&card| Self::get_card_rank(card) == Some(rank as u32))
                .cloned()
                .collect();
            return (true, three_of_a_kind_cards);
        }
        (false, sorted_hand)
    }

    fn get_two_pairs(hand: &Vec<Card>) -> (bool, Vec<Card>) {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        let ranks: Vec<u32> = sorted_hand
            .iter()
            .map(|card| Self::get_card_rank(card).unwrap_or_default())
            .collect();
        let mut counts: Vec<usize> = vec![0; 15];
        for &rank in ranks.iter() {
            counts[rank as usize] += 1;
        }
        let mut pairs: Vec<Card> = vec![];
        for (count, rank) in counts.iter().enumerate().skip(2) {
            if *rank == 2 {
                let pair_cards: Vec<Card> = sorted_hand
                    .iter()
                    .filter(|&card| Self::get_card_rank(card) == Some(count as u32))
                    .cloned()
                    .collect();
                pairs.extend(pair_cards);
            }
        }
        if pairs.len() >= 4 {
            pairs.sort_by_key(|card| Self::get_card_rank(card).unwrap_or_default());
            return (true, pairs);
        }
        (false, sorted_hand)
    }

    
    fn get_one_pair(hand: &Vec<Card>) -> (bool, Vec<Card>) {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        let ranks: Vec<u32> = sorted_hand
            .iter()
            .map(|card| Self::get_card_rank(card).unwrap_or_default())
            .collect();
        let mut counts: Vec<usize> = vec![0; 15];
        for &rank in ranks.iter() {
            counts[rank as usize] += 1;
        }
        if let Some(rank) = counts.iter().position(|&count| count == 2) {
            let pair_cards: Vec<Card> = sorted_hand
                .iter()
                .filter(|&card| Self::get_card_rank(card) == Some(rank as u32))
                .cloned()
                .collect();
            return (true, pair_cards);
        }
        (false, sorted_hand)
    }
    

    fn get_royal_flush(hand: &Vec<Card>) -> (bool, Vec<Card>) {
        let (is_flush, sorted_hand) = Self::get_flush(hand);
        if is_flush {
            let ranks: Vec<u32> = sorted_hand
                .iter()
                .map(|card| Self::get_card_rank(card).unwrap_or_default())
                .collect();
            let mut distinct_ranks: Vec<u32> = ranks.clone();
            distinct_ranks.dedup();
            if distinct_ranks == vec![10, 11, 12, 13, 14] {
                return (true, sorted_hand);
            }
        }
        (false, sorted_hand)
    }

    fn get_kicker(hand: &Vec<Card>, exclude_rank: &str) -> (Option<u32>, Option<Card>) {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        sorted_hand.reverse();
        for card in &sorted_hand {
            if card.get_rank() != exclude_rank {
                return (Self::get_card_rank(&card), Some(card.clone()));
            }
        }
        (None, None)
    }

    fn get_high_card_rank(cards: &Vec<Card>) -> (Option<u32>, Option<Card>) {
        let mut sorted_cards = cards.clone();
        sorted_cards.sort_by_key(|card| String::from(card.get_rank()).clone());
        sorted_cards.reverse();
        if let Some(card) = sorted_cards.first() {
            return (Self::get_card_rank(&card), Some(card.clone()));
        } else {
            (None, None)
        }
    }

    fn get_card_rank(card: &Card) -> Option<u32> {
        match String::from(card.get_rank()).parse::<u32>() {
            Ok(rank) => Some(rank),
            Err(_) => match card.get_rank() {
                "Ace" => Some(14),
                "King" => Some(13),
                "Queen" => Some(12),
                "Jack" => Some(11),
                "10" => Some(10),
                _ => None,
            },
        }
    } 

    fn get_sorted_cards(&self, hand: &mut Vec<Card>) -> Vec<Card> {
        hand.sort_by_key(|card| String::from(card.get_rank()).clone());
        hand.to_vec()
    }

    fn get_hand_rank(&self, player: Player, dealer: &Dealer, community_deck: &Vec<Card>) -> Player {
        let mut current_player = player;
        let mut game_dealer = dealer.clone();
        let community_cards = community_deck.clone();
        let player_cards = game_dealer.request_player_hand(&current_player).unwrap_or_default();
        let mut collected_hand_cards = Vec::new();

        if player_cards.len() > 0 {
            let player_cards_for_extend = player_cards.clone();
            collected_hand_cards.extend(player_cards_for_extend);
        }

        collected_hand_cards.extend(community_cards);

        let (is_royal_flush, is_royal_flush_winning_cards) = Self::get_royal_flush(&collected_hand_cards);
        let (is_full_house, is_full_house_winning_cards) = Self::get_full_house(&collected_hand_cards);
        let (is_four_of_a_kind, is_four_of_a_kind_winning_cards) = Self::get_four_of_a_kind(&collected_hand_cards);
        let (is_flush, is_flush_winning_cards) = Self::get_flush(&collected_hand_cards);
        let (is_straight, is_straight_winning_cards) = Self::get_straight(&collected_hand_cards);
        let (is_three_of_a_kind, is_three_of_a_kind_winning_cards) = Self::get_three_of_a_kind(&collected_hand_cards);
        let (is_two_pairs, is_two_pairs_winning_cards) = Self::get_two_pairs(&collected_hand_cards);
        let (is_one_pair, is_one_pair_winning_cards) = Self::get_one_pair(&collected_hand_cards);
        let high_card_rank = Self::get_high_card_rank(&collected_hand_cards);
        let kicker_card_rank = Self::get_kicker(&collected_hand_cards, high_card_rank.0.unwrap().to_string().as_str());
        
        if is_royal_flush {
           current_player.set_handrank( HandRank {
                hand_rank: 9,
                hand_rank_name: "Royal Flush".to_string(),
                winning_cards: is_royal_flush_winning_cards.clone(),
                high_card: is_royal_flush_winning_cards[0].clone(),
                kicker: kicker_card_rank.1.unwrap()
           });

           return current_player;
        }

        if is_straight && is_flush {
            current_player.set_handrank( HandRank {
                 hand_rank: 8,
                 hand_rank_name: "Straight Flush".to_string(),
                 winning_cards: is_straight_winning_cards.clone(),
                 high_card: is_straight_winning_cards[0].clone(),
                 kicker: kicker_card_rank.1.unwrap()
            });
 
            return current_player;
         }
       
        if is_four_of_a_kind {
            current_player.set_handrank( HandRank {
                hand_rank: 7,
                hand_rank_name: "Four of a Kind".to_string(),
                winning_cards: is_four_of_a_kind_winning_cards.clone(),
                high_card: is_four_of_a_kind_winning_cards[0].clone(),
                kicker: kicker_card_rank.1.unwrap()
            });

            return current_player;
        }

        if is_full_house {
            current_player.set_handrank( HandRank {
                hand_rank: 6,
                hand_rank_name: "Full House".to_string(),
                winning_cards: is_full_house_winning_cards.clone(),
                high_card: is_full_house_winning_cards[0].clone(),
                kicker: kicker_card_rank.1.unwrap()
            });

            return current_player;
        }

        if is_flush {
           current_player.set_handrank( HandRank {
                hand_rank: 5,
                hand_rank_name: "Flush".to_string(),
                winning_cards: is_flush_winning_cards.clone(),
                high_card: is_flush_winning_cards[0].clone(),
                kicker: kicker_card_rank.1.unwrap()
           });

           return current_player;
        }

        if is_straight {
            current_player.set_handrank( HandRank {
                hand_rank: 4,
                hand_rank_name: "Straight".to_string(),
                winning_cards: is_straight_winning_cards.clone()[0..5].to_vec(),
                high_card: is_straight_winning_cards[0].clone(),
                kicker: kicker_card_rank.1.unwrap()
            });

            return current_player;
        }

        if is_three_of_a_kind {
            current_player.set_handrank( HandRank {
                hand_rank: 3,
                hand_rank_name: "Three of a Kind".to_string(),
                winning_cards: is_three_of_a_kind_winning_cards.clone(),
                high_card: is_three_of_a_kind_winning_cards[0].clone(),
                kicker: kicker_card_rank.1.unwrap()
            });

            return current_player;
        }

        if is_two_pairs {
            current_player.set_handrank( HandRank {
                hand_rank: 2,
                hand_rank_name: "Two Pairs".to_string(),
                winning_cards: is_two_pairs_winning_cards.clone(),
                high_card: is_two_pairs_winning_cards[0].clone(),
                kicker: kicker_card_rank.1.unwrap()
            });

            return current_player;
        }

        if is_one_pair {
            current_player.set_handrank( HandRank {
                hand_rank: 1,
                hand_rank_name: "One Pair".to_string(),
                winning_cards: is_one_pair_winning_cards.clone(),
                high_card: is_one_pair_winning_cards[0].clone(),
                kicker: kicker_card_rank.1.unwrap()
            });

            return current_player;
        }

        current_player.set_handrank( HandRank {
            hand_rank: 0,
            hand_rank_name: "High Card".to_string(),
            winning_cards: player_cards.clone(),
            high_card: player_cards[0].clone(),
            kicker: kicker_card_rank.1.unwrap()
        });


        current_player
    }

}
