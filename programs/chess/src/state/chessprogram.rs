#[account]
#[derive(InitSpace)]
pub struct ChessProgram{
    pub admin:Publickey,
    pub game_count:u64,
}