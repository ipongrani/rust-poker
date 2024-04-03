// Define the possible actions and the ActionResult struct
use crate::constants::PlayerAction;



#[derive(Debug)]
pub struct ActionResult {
    pub action: PlayerAction,
    pub success: bool,
    pub amount: Option<u32>
}