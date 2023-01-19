use anchor_lang::prelude::*;

#[account]
pub struct Arena {
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub round: u8,
    pub is_ready: bool,
    pub p1_escrow: Pubkey,
    pub p2_escrow: Pubkey,
    pub warrior_metadata1: Pubkey,
    pub warrior_metadata2: Pubkey,
}
