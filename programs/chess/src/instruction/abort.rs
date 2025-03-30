use anchor_lang::prelude::*;
use crate::{
    state::game::{Game,Status,Timecontrol}
};
use anchor_lang::prelude::Clock;
pub mod abort_game{
    use super::*;

    pub fn abort_game(ctx:Context<AbortGame>,
        status:u8,
        time_control:u64,
        end_reason:Option<EndReason>,
        moves:String)->Result<()>{
let  abort_game = &mut ctx.accounts.game_account;
game.end_reason = EndReason::EndReason3;
game.status = Status::Aborted;

require!(game.status == Status::Waiting , ErrorCode::GameAlreadyActive);

require!(game.status == Status::Active , ErrorCode::GameAlreadyActive);

game.time_control = TimeControl::CreatedAt(Clock::get()?.unix_timestamp);

require!(clock - game.time_control >= 30 , ErrorCode::TimeOut);

if game.stake >0{
**ctx.accounts.host.to_account_info().try_borrow_mut_lamports()? += game.stake;
**ctx.accounts.game_account.to_account_info().try_borrow_mut_lamports()? -= game.stake;
};
}
}

#[derive(Accounts)]
pub struct AbortGame<'info>{
     #[account(mut)]
    pub host:Signer<'info>,
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