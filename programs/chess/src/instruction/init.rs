use anchor_lang::prelude::*;
use crate::state::ChessProgram;
pub mod init{
    use super::*;

    pub fn init(ctx:Context<Init>,
                adim:Pubkey,
                _game_count:u64,
                )->Result<()>{
            let init_game = &mut ctx.accounts.chess_program;
            chess_program.new(
                ctx.accounts.admin.key(),
                game_count,
            )
        Ok(())
    }
}

pub struct Init<'info>{
    #[account(mut)]
    pub admin : Signer<'info>,
    #[account(
        init,
        payer=payer,
        space = 8+ChessProgram::INIT_SPACE
    )]
    pub chess_program : Account<'info,ChessProgram>,

    pub system_program: Program<'info,System>,
}