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

    pub fn start_game(&mut self, opponent:Pubkey, stake:u64)-> Self{
        self.opponent = Some(opponent);
        self.stake = stake;
        self.status = GameStatus::Active;
        self.player_turn = opponent;
    }
     pub fn end_game(&mut self, winner:Pubkey, end_reason:Option<EndReason>)-> Self{
        self.winner = Some(winner);
        self.end_reason = Some(EndReason);
        self.status = GameStatus::Completed;
     }

     pub fn abort_game(&mut self, end_reason:Option<EndReason3>)-> Self{
        self.end_reason = Some(end_reason);
        self.status = GameStatus::Aborted;
     }
     pub fn withdraw(&mut self, winner:Pubkey , game_id:Pubkey)-> Self{
        self.game_id = game_id;
        self.winner = Some(winner);
        self.status = GameStatus::Completed;
     }
     pub fn make_move(&mut self, move_data:MoveData)-> Self{
        self.moves.push(move_data);
        self.status = GameStatus::Active;
        self.player_turn = if self.player_turn == self.host {
            self.opponent.unwrap()
        } else {
            self.host
        };
     }
}