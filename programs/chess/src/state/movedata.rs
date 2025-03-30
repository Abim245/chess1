#[account]
pub struct MoveData{
    pub host: Pubkey,
    pub from: (u8,u8),
    pub to: (u8,u8),
}