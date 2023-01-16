use super::Trait;
use anchor_lang::prelude::*;

#[account]
pub struct WarriorMetadata {
    pub mint: Pubkey,
    pub num_of_victories: u8,
    //Stats
    pub attack: u8,
    pub defense: u8,
    pub armor: u8,
    pub armor_pen: u8,
    //Special traits
    pub background: Trait,
    pub helmet: Trait,
    pub body: Trait,
    pub hand: Trait,
    pub weapon: Trait,
}
