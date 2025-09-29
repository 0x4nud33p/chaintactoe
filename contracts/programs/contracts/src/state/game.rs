use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub player_x: Pubkey,
    pub player_o: Pubkey,
    pub board: [u8; 9],     // 0 empty, 1 X, 2 O
    pub turn: u8,           // 1 = X, 2 = O
    pub status: u8,         // 0 open, 1 X won, 2 O won, 3 draw
    pub stake: u64,         // lamports per player
    pub created_at: i64,
    pub last_move_ts: i64,
    pub timeout_seconds: u64,
    pub bump: u8,
}
