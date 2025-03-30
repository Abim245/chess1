use anchor_lang::prelude::*;
use crate::{
    state::game::{Game,Status,Timecontrol}
};
use anchor_lang::prelude::Clock;

pub mod make_move{
    use super::*;

    pub fn make_move(ctx:Context<MakeMove>,
        moves:Vec<MoveData>,
        last_black_move:i64,
        last_white_move:i64,
        player_turn:Pubkey,
        status:u8)->Result<()>{
let make_move = &mut ctx.account.game_account;


game.status = Status::Active;
require!(game.status == Status::Active, ErrorCode = GameNotActive);

require!(game.player_turn == host.key(), ErrorCode=NotYourTurn);
game.player_turn = match game.player_turn{
p if p == game.host => game.opponent.unwrap(),
_ => game.host,
};
//record move
game.moves.push(MoveData{
player: host.key(),
from,
to,
});
//update last move
game.last_black_move = Clock::get()?.unix_timestamp;
game.last_white_move = Clock::get()?.unix_timestamp;
//switch to other players
if game.player_turn == game.host{
game.player_turn = game.opponent;
}else{
game.player_turn = game.host;
}


Ok(())

}
}


#[derive(Accounts)]
pub struct MakeMove<'info>{
    #[account(mut)]
    pub host:Signer<'info>,
    pub opponent:AccountInfo<'info>,
    #[account(
        mut,
        seeds=[b"game",game_pda.key.as_ref()],
        bump
    )]
    pub game_pda:Account<'info,GamePda>,
    #[account(mut)]
    pub game_account: Account<'info , Game>,
    pub system_program: Program<'info, System>
}
