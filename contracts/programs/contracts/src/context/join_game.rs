use anchor_lang::prelude::*;
use crate::state::game::Game;
use crate::errors::CustomError;
use crate::events::GameJoined;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player_o: Signer<'info>,

    #[account(mut, has_one = player_x)]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<JoinGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    require!(game.status == 0, CustomError::GameNotOpen); // 0 open, 1 X won, 2 O won, 3 draw
    require!(game.player_o == Pubkey::default(), CustomError::GameAlreadyHasPlayerO);
    game.player_o = ctx.accounts.player_o.key();
    game.status = 1; // game started, X's turn

    let cpi_accounts = Transfer {
        from: ctx.accounts.player_o.to_account_info(),
        to: ctx.accounts.game.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, game.wager)?;
    
    game.total_pot += game.wager;

    emit!(GameJoined {
        game: game.key(),
        player_o: game.player_o,
        joined_at: Clock::get().unwrap().unix_timestamp,
    });

    Ok(())
}