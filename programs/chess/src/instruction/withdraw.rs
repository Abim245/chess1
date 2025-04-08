use anchor_lang::prelude::*;
use crate::{
    state::game::{Game,Status,Timecontrol}
};
use anchor_lang::prelude::Clock;

pub mod withdraw{
    use super::*;e anchor_lang::prelude::*;
use crate::{
    state::game::{Game,Status,Timecontrol}
};
use anchor_lang::prelude::Clock;

    pub fn withdraw(ctx:Context<WithdrawAmount>,
        status:u8,
        game_id:Pubkey,
        winner:Pubkey)->Result<()>{
let withdraw = &mut ctx.accounts.game_account;
// game.game_id = &ctx.accounts.game_id.key();
// game.status = Status::Completed;
game.withdraw(
    winner,
    game_id,
    status,
)
if game.winner != game.winner{
    return Err(ChessError::NotWinner.into());
}

game.winner = Some(winner);
let stake = game.stake*2;

invoke(
&system_instruction::transfer(
    &game.to_account_info().key(),
    &winner,
    stake,
),
&[
    game.to_account_info(),
    ctx.accounts.winner_account.to_account_info(),
    system_program.to_account_info(),
]
)?;
game.winner =None;
let stake = game.stake;

invoke(
&system_instruction::transfer(
    &game.to_account_info().key(),
    &game.host,
    stake,
),
&[
    game.to_account_info(),
    ctx.accounts.host_account.to_account_info(),
    system_program.to_account_info(),
],
)?;

invoke(
&system_instruction::transfer(
    &game.to_account_info().key(),
    &game.opponent.unwrap(),
    stake,
),
&[
    game.to_account_info(),
    ctx.accounts.opponent_account.to_account_info(),
    system_program.to_account_info(),
]
)
}
}

#[derive(Accounts)]
pub struct WithdrawAmount<'info>{
     #[account(mut)]
    pub winner:Signer<'info>,
    #[account(mut)]
    pub game_account: Account<'info , Game>,
    #[account(
        mut,
        seeds=[b"game",game_pda.key.as_ref()],
        bump
    )]
    pub game_pda:Account<'info,GamePda>,
    pub system_program: Program<'info, System>
}