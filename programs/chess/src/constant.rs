se anchor_lang::prelude::*;

pub fn game_pda(game_id:Pubkey)->(Pubkey,u8){
    Pubkey::find_program_address(&[b"game",game_id.as_ref()],&ID)}