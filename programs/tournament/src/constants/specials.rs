use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Special {
    //Background -> Match wide buffs
    Grassland,   // Attack twice in the first round
    Battlefield, // Execute enemy when two rounds in a row he is below 30%
    Castle,      // Opponent doesnt attack first and last round
    Hell,        // Revive with 30% hp on first death

    //Helmet ->  Compensation self buffs
    Iron,   // If your ATK is lower than opponents DEF, gain more ATK
    Bronze, // If your AP is lower than opponents AR, gain more ATK
    Gold,   // If your AR is lower than opponents AP, gain more DEF
    Jade,   // If your DEF is lower than opponents ATK, gain more DEF

    //Body ->
    Plate,
    Leather,
    Emerald,
    PLACEHOLDER,

    //Hand -> Passive on each "round"
    Protected, // Every time you take dmg, opponent takes 30% of that dmg too
    Fiery,     // Burn 5% of opponents max health on each attack
    Bloody,    // Lifesteal: Heal 30% of all dmg you deal
    Bare,      // Gain AP based on your DEF/AR

    //Weapon -> Snowballng self buffs
    Halberd,      // If your ATK is higher than opponents ATK, gain more atk
    Sword,        // If your AP is higher than opponents AP, gain more ap
    Shield,       // If your DEF is higher than opponents DEF, gain more DEF
    SpikedShield, // If your AR is higher opponents AR, gain more AR
}
