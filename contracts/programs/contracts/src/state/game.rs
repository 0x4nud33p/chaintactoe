use anchor_lang::prelude::*;

#[account]
#[cfg(test)]
mod game_tests;

pub struct Game {
    pub player_x: Pubkey,
    pub player_o: Pubkey,
    pub wager: u64,             // lamports per player
    pub board: [u8; 9],         // 0 empty, 1 X, 2 O
    pub turn: u8,               // 1 = X, 2 = O
    pub status: u8,             // 0 open, 1 X won, 2 O won, 3 draw
    pub total_pot: u64,         // total amount in the pot
    pub created_at: i64,
    pub last_move_ts: i64,
    pub timeout_seconds: u64,    // time allowed per move
    pub bump: u8,
}
