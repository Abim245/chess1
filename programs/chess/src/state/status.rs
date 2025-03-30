use anchor_lang::prelude::*;

pub enum GameStatus{
    Waiting,
    Active,
    Completed,
    Aborted
}

pub struct Waiting{
    pub host: Pubkey,
    pub stake:u64,
    pub time_control:u64,
}

pub struct Active{
    pub opponent:Pubkey,
    pub stake:u64,
    pub created_at:i64,
}

pub struct Completed{
    pub last_white_move:i64,
    pub last_black_move:i64,
    pub winner:Pubkey,
}

pub struct Aborted{
    pub end_reason:String,
}