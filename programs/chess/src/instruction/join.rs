use anchor_lang::prelude::*;
use crate::{
    state::game::{Game,Status,Timecontrol}
};
use anchor_lang::prelude::Clock;

pub fn join_game(ctx:Context<JoinGame>,
                 game_id:Pubkey,
                 status:u8,
                 opponent:Pubkey)->Result<()>{
    let join_game = &mut ctx.account.game_account;

    // game.game_id = ctx.account.host.key();
    // game.status= Status::Active;
    // game.opponent = &mut ctx.account.opponent.key();
    // game.stake = stake;

    game.start_game(ctx.accounts.opponent.key(),
                    ctx.accounts.game_account.stake,
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
    Ok(())
}


#[derive(Accounts)]
pub struct JoinGame <'info>{
    #[account(mut)]
    pub opponent:Signer<'info>,
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