
mod player;
mod implementations;
mod features;
mod constants;
mod dealer;
mod card;
mod poker_game;



use player::Player;
use dealer::Dealer;
use poker_game::PokerGame;

use std::time::Instant;



fn main() {
    let start_time = Instant::now();
    let dealer = Dealer::new("auto".to_string());
    let player1 = Player::new("AutoPlayer", 1, String::from("Alice"), 100);
    let player2 = Player::new("AutoPlayer", 2, String::from("Bob"), 100);
    let player3 = Player::new("AutoPlayer", 3, String::from("Charlie"), 100);
    let player4 = Player::new("AutoPlayer", 4, String::from("Dave"), 100);
    let player5 = Player::new("AutoPlayer", 5, String::from("Eve"), 100);
    let mut game = PokerGame::new(vec![player1, player2, player3, player4, player5], dealer);
    //game.play();
    game.request_generate_deck(1);
    game.request_dealer_shuffle();
    game.play_with_blinds(true);
    //game.play_with_blinds(false);
    game.post_blinds_current_game(5);
    game.deal_cards();
    game.play();
    //game.post_blinds_current_game(5);
    //game.show_status();
    let duration = start_time.elapsed();
    let milliseconds = duration.as_millis();

    println!("Execution Time: {}~ms", milliseconds);
}
