use anchor_lang::prelude::*;
use crate::state::game::Game;

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player_o: Signer<'info>,

    #[account(mut, has_one = player_x)]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
}
