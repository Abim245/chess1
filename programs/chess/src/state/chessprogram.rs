#[account]
#[derive(InitSpace)]
pub struct ChessProgram{
    pub admin:Publickey,
    pub game_count:u64,
}
impl ChessProgram{
    pub fn new(admin:Pubkey)-> Self{
        Self{
            admin,
            game_count:0,
        }
    }
}