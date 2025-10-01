use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("The game is not open for joining.")]
    GameNotOpen,
    #[msg("The game already has a player O.")]
    GameAlreadyHasPlayerO,
}