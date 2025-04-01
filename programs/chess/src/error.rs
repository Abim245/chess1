use anchor_lang::prelude::*;

#[error_code]
pub enum ChessError {
    #[msg("inactive opponent")]
    InactivePlayer,

    #[msg("insufficient amount")]
    InsufficientFunds,

    #[msg("invalid game")]
    InvalidGame,

    #[msg("invalid move")]
    InvalidMove,

    #[msg("not player turn")]
    NotPlayerTurn,

    #[msg("time out")]
    Timeout,
    
    #[msg("not a winner")]
    NotWinner,}