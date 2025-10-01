use anchor_lang::prelude::*;

#[event]
pub struct GameCreated {
    pub game: Pubkey,
    pub player_x: Pubkey,
    pub wager: u64,
    pub created_at: i64,
}

#[event]
pub struct GameJoined {
    pub game: Pubkey,
    pub player_o: Pubkey,
    pub joined_at: i64,
}