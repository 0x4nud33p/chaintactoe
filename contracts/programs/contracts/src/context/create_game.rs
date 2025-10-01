use anchor_lang::prelude::*;
use crate::state::game::*;

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Game::LEN,
    )]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateGame>, wager: u64) -> Result<()> {
    let game = &mut ctx.accounts.game;
    game.player_x = ctx.accounts.creator.key();
    game.wager = wager;
    game.board = [0; 9];
    game.turn = 1;
    game.status = 0;
    game.created_at = Clock::get().unwrap().unix_timestamp;
    game.last_move_ts = game.created_at;
    game.timeout_seconds = 120; // 2 minutes
    game.bump = *ctx.bumps.get("game").unwrap();

    emit!(GameCreated {
        game: game.key(),
        player_x: game.player_x,
        wager: game.wager,
        created_at: game.created_at,
    });

    Ok(())
}
