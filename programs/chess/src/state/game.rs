#[account]
#[derive(InitSpace)]
pub struct Game{
    pub game_id:Pubkey,
    pub host:Pubkey,
    pub opponent:Option<Pubkey>,
    pub winner: Option<Pubkey>,
    pub stake:u64,
    pub end_reason:Option<EndReason>,
    #[max_len(32)]
    pub moves:Vec<MoveData>,
    pub time_control:u64,
    pub status:u8,
    pub player_turn:Pubkey,
    pub last_black_move : i64,
    pub last_white_move: i64,
}

impl Game {
    pub fn new_game(game_id:Pubkey, host:Pubkey, stake:u64, time_control:u64) -> Self{
        Self{
            game_id,
            host,
            opponent:None,
            winner:None,
            stake,
            end_reason:None,
            moves:Vec::new(),
            time_control,
            status : GameStatus::Waiting,
            player_turn:host,
            last_white_move:0,
            last_black_move:0,
        }

    }

    pub fn start_game(&mut self, opponent:Pubkey, stake:u64, status:u8)-> Self{
        self.opponent = Some(opponent);
        self.stake = stake;
        self.status = GameStatus::Active;
        ()
    }
     pub fn end_game(&mut self, winner:Pubkey, end_reason:Option<EndReason>, status:u8)-> Self{
        self.winner = Some(winner);
        self.end_reason = end_reason;
        self.status = GameStatus::Completed;
        ()
     }

}