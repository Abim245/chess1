pub enum EndReason{
    claim_victory(EndReason1),
    claim_draw(EndReason2),
    abort_game(EndReason3),
}
pub enum EndReason1{
    checkmate,
    timestamp,
    opponent,
}

pub enum EndReason2{
    stalemate,
    agreement,
    insufficient_material,
}

pub enum EndReason3{
    inactivity,
    aggreement,
}