use anchor_lang::prelude::*;
use crate::state::game::Game;
use crate::errors::CustomError;


#[derive(Accounts)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

pub fn make_move(ctx: Context<MakeMove>, x: u8, y: u8) -> Result<(), CustomError> {
    let game = &mut ctx.accounts.game;
    let player = &ctx.accounts.player;

    require!(game.status == 1 || game.status == 2, CustomError::GameCompleted);
    require!(player.key() == game.player_x || player.key() == game.player_o, CustomError::PlayerNotMatched);
        let is_player_x = player.key() == game.player_x;
        let is_player_o = player.key() == game.player_o;
        require!(is_player_x || is_player_o, CustomError::PlayerNotMatched);
        if game.status == 1 {
            require!(is_player_x, CustomError::PlayerNotMatched);
        } else if game.status == 2 {
            require!(is_player_o, CustomError::PlayerNotMatched);
        }
    require!(x < 3 && y < 3, CustomError::PlayerNotMatched);
    let idx = (x * 3 + y) as usize;
    require!(game.board[idx] == 0, CustomError::PlayerNotMatched);
    game.board[idx] = if is_player_x { 1 } else { 2 };
    // Check for win or draw
    let win_conditions = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
        [0, 4, 8], [2, 4, 6]             // diagonals
    ];
    for condition in win_conditions.iter() {
        if game.board[condition[0]] != 0 &&
           game.board[condition[0]] == game.board[condition[1]] &&
           game.board[condition[1]] == game.board[condition[2]] {
            game.status = if game.board[condition[0]] == 1 { 3 } else { 4 }; // 3 X won, 4 O won
            return Ok(());
        }
    }
    if !game.board.contains(&0) {
        game.status = 5; // 5 draw
    } else {
        game.status = if game.status == 1 { 2 } else { 1 }; // switch turns
    }

    emit!(MoveMade {
        game: game.key(),
        player: player.key(),
        x,
        y,
        made_at: Clock::get().unwrap().unix_timestamp,
    });

    Ok(())
}
