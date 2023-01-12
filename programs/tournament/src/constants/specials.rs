use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Special {
    //Background
    Grassland,
    Battlefield,
    Castle,
    Sky,

    //Helmet
    Iron,
    Bronze,
    Gold,
    None,

    //Body
    Plate,
    Leather,
    PLACEHOLDER1,
    PLACEHOLDER2,

    //Hand
    Protected,
    Fiery,
    Bloody,
    Bare,

    //Weapon
    Spear,
    Sword,
    Shield,
    SpikedShield,
}
