#[derive(Debug, Clone)]
pub enum PlayerAction {
    Check,
    Bet,
    Raise,
    Call,
    Fold,
}