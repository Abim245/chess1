use anchor_lang::prelude::*;
use crate::{
    state::game::{Game,Status,Timecontrol}
};
use anchor_lang::prelude::Clock;

pub mod claim_victory{
    use super::*;

    pub fn claim_victory(ctx:Context<ClaimVictory>,
        status:u8,
        last_white_move:i64,
        last_black_move:i64,
        end_reason:Option<EndReason>,
        time_control:u64,
        winner:Pubkey)->Result<()>{
let claim_victory = &mut ctx.accounts.game_account;

// game.end_reason = EndReason::EndReason1;
// game.status =Status::Completed;
// game.winner = &ctx.accounts.winner.key();
game.end_game(ctx.accounts.host.key(),
              end_reason,
              status);
    
game.time_control = TimeControl::EndTime(Clock::get()?.unix_timestamp);


 }
}

#[derive(Accounts)]
pub struct ClaimVictory<'info>{
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