use anchor_lang::prelude::*;

use crate::{
    state::game::{Game,Status,Timecontrol}
};
use anchor_lang::prelude::Clock;

pub mod create{
    use super::*;

    pub fn create(ctx:Context<CreateGame>, 
        stake:u64, 
        time_control:u64,
        status:u8)->Result<()>{
let create_game = &mut ctx.account.game_account;

// game.host= &mut ctx.account.host.key();
// game.stake = stake;
game.new_game(ctx.accounts.host.key,
            ctx.Account.game_account.stake,
            time_control,
            status);

if game.stake > 0{
    let transfer_instruction = system_instruction::transfer{
     opponent.to_account_info().key(),
     game_account.to_account_info().key(),
     stake,
};

invoke(
    &transfer_instruction,
    &[
        opponent.to_account_info(),
        ctx.accounts.game_account.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ],
)?;
} else{
    return Err(ChessError::InsufficientFunds.into());
}
game.time_control = TimeControl::StartTime(Clock::get()?.unix_timestamp);
game.status = Status::Waiting;

Ok(())
}
}

#[derive(Accounts)]
pub struct CreateGame<'info>{
    #[account(mut)]
    pub host:Signer<'info>,
    #[account(
        init,
        payer=host,
        space=8+Game::INIT_SPACE,
        seeds=[b"game",host.key().as_ref()],
        bump,
    )]
    pub game_account: Account<'info , Game>,
    pub system_program: Program<'info, System>,
}
 



 

