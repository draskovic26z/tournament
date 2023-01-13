use anchor_lang::prelude::*;

use crate::constants::Special;

#[account]
pub struct WarriorMetadata {
    pub mint: Pubkey,
    pub num_of_victories: u8,
    //Stats
    pub attack: u8,
    pub defense: u8,
    pub armor: u8,
    pub attack_speed: u8,
    //Special traits
    pub background: Special,
    pub helmet: Special,
    pub body: Special,
    pub hand: Special,
    pub weapon: Special,
}
