use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Special {
    //Background
    Grassland,
    Battlefield,
    Castle, // Opponent doesnt attack first round
    Hell,   // On first death, revive with 30hp

    //Helmet
    Iron,
    Bronze,
    Gold,
    Jade,

    //Body
    Plate,
    Leather,
    Emerald,
    PLACEHOLDER2,

    //Hand
    Protected,
    Fiery,
    Bloody,
    Bare,

    //Weapon
    Halberd,
    Sword,
    Shield,
    SpikedShield,
}
