use anchor_lang::prelude::*;

#[account]
pub struct WarriorMetadata {
    pub authority: Pubkey, // same as metadata update authority
    pub mint: Pubkey,
    pub num_of_victories: u8,
    pub attack: u8,
    pub defense: u8,
    pub armor: u8,
    pub attack_speed: u8,
}
