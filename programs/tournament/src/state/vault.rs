use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub round: u8,
    pub is_ready: bool,
    pub p1_token_account: Pubkey,
    pub p2_token_account: Pubkey,
}
