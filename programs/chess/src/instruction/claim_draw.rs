use anchor_lang::prelude::*;
use crate::{
    state::game::{Game,Status,Timecontrol}
};
use anchor_lang::prelude::Clock;
pub mod claim_draw{
    use super::*;

   pub fn claim_draw(ctx:Context<ClaimDraw>,
        status:u8,
        end_reason:Option<EndReason>)->Result<()>{
let claim_draw = &mut ctx.accounts.game_account;

// game.status = Status::Completed;
// game.winner = None;
// game.end_reason = Some(reason);

game.end_game(None,
              end_reason = Some(EndReason::EndReason2),
              status);
    


Ok(())
 }
}

#[derive(Accounts)]
pub struct ClaimDraw<'info>{
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