use anchor_lang::prelude::*;
use crate::state::game::Game;

#[derive(Accounts)]
pub struct FinalizeGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

pub fn finalize_game(ctx: Context<FinalizeGame>) -> Result<(), CustomError> {
    let game = &mut ctx.accounts.game;
    let player = &ctx.accounts.player;

    require!(game.status == 3 || game.status == 4, CustomError::GameNotCompleted);
    require!(player.key() == game.player_x || player.key() == game.player_o, CustomError::PlayerNotMatched);

    // Distribute winnings
    if game.status == 3 {
        // Player X won
        let fee = game.total_pot / 20; // 5% fee
        let cpi_accounts = Transfer {
            from: ctx.accounts.game.to_account_info(),
            to: ctx.accounts.player.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, game.total_pot - fee)?;
        emit!(FeeCollected {
            game: game.key(),
            fee,
            collected_at: Clock::get().unwrap().unix_timestamp,
        });

        emit!(WinningsDistributed {
            game: game.key(),
            winner: game.player_x,
            amount: game.total_pot - fee,
            distributed_at: Clock::get().unwrap().unix_timestamp,
        });

    } else if game.status == 4 {
        // Player O won
        let fee = game.total_pot / 20; // 5% fee
        let cpi_accounts = Transfer {
            from: ctx.accounts.game.to_account_info(),
            to: ctx.accounts.player.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, game.total_pot - fee)?;
        emit!(FeeCollected {
            game: game.key(),
            fee,
            collected_at: Clock::get().unwrap().unix_timestamp,
        });
        emit!(WinningsDistributed {
            game: game.key(),
            winner: game.player_o,
            amount: game.total_pot - fee,
            distributed_at: Clock::get().unwrap().unix_timestamp,
        });
    }

    emit!(GameFinalized {
        game: game.key(),
        winner: if game.status == 3 { game.player_x } else { game.player_o },
        finalized_at: Clock::get().unwrap().unix_timestamp,
    });

    Ok(())
}
