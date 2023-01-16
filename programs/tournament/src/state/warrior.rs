use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Debug)]
pub struct Warrior {
    pub damage: i32,
    pub hp: i32,
    pub armor: i32,
    pub armor_pen: i32,
    pub background: Trait,
    pub helmet: Trait,
    pub body: Trait,
    pub hand: Trait,
    pub weapon: Trait,
    pub atk: u8,
    pub def: u8,
    pub ar: u8,
    pub ap: u8,
    pub won: bool,
}

#[derive(Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum Trait {
    //Background
    Grassland,   // Attack three times in the first round
    Battlefield, // Execute the enemy after one round when he gets really low on hp
    Castle,      // Enemy doesn't attack first and last round
    Hell,        // Revive with a portion of your hp on first death

    //Helmet
    Hood,   // If your ATK is lower than opponents DEF, gain more ATK
    Raider, // If your AP is lower than opponents AR, gain more ATK
    Iron,   // If your AR is lower than opponents AP, gain more DEF
    Skull,  // If your DEF is lower than opponents ATK, gain more DEF

    //Body
    Plate,   // The longer the match lasts the more AR you gain
    Leather, // The longer the match lasts the more AP you gain
    Golden, // Gain a AR buff at the beginning of the match, that grows weaker the longer the match goes on
    Demonic, // Gain an AP buff at the beginning of the match, that grows weaker the longer the match goes on

    //Hand
    Protected, // Every time you take dmg, the enemy takes a portion of that damage too
    Crimson,   // Burn a portion of enemies max health on each attack
    Bloody,    // Lifesteal: Heal for a portion of all dmg you deal
    Bare,      // Deny opponents hand trait

    //Weapon
    Halberd, // If your ATK is higher than opponents ATK, gain more atk
    Sword,   // If your AP is higher than opponents AP, gain more ap
    Shield,  // If your DEF is higher than opponents DEF, gain more DEF
    Hammer,  // If your AR is higher opponents AR, gain more AR
}

impl Warrior {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        atk: u8,
        def: u8,
        ar: u8,
        ap: u8,
        background: Trait,
        helmet: Trait,
        body: Trait,
        hand: Trait,
        weapon: Trait,
    ) -> Warrior {
        Warrior {
            damage: 150 + atk as i32 * 25,
            hp: 1500 + def as i32 * 150,
            armor: ar as i32 * 100,
            armor_pen: ap as i32 * 100,
            background,
            helmet,
            body,
            hand,
            weapon,
            atk,
            def,
            ar,
            ap,
            won: false,
        }
    }

    pub fn attack(&mut self, enemy: &mut Warrior) -> i32 {
        let enemy_hp = enemy.hp;
        let dmg_done = self.damage * (1000 - enemy.armor + self.armor_pen) / 1000;
        enemy.hp -= dmg_done;
        if enemy.armor - self.armor_pen > 0 {
            println!("Damage mitigated: {:?}", self.damage - dmg_done);
        } else {
            println!("Bonus damage: {:?}", dmg_done - self.damage);
        }

        if self.hand != Trait::Bare && enemy.hand == Trait::Protected {
            println!(
                "*{:?} damage taken from enemies protected hand*",
                dmg_done / 3
            );
            self.hp -= dmg_done / 3;
        }

        if self.hand == Trait::Crimson && enemy.hand != Trait::Bare {
            println!(
                "*Burns enemy HP for {:?}*",
                (enemy.def as i32 * 200 + 1100) * 50 / 1000
            );
            enemy.hp -= (enemy.def as i32 * 200 + 1100) * 50 / 1000
        }

        if self.hand == Trait::Bloody && enemy.hand != Trait::Bare {
            println!("*Lifesteal for {:?}*", dmg_done / 3);
            self.hp += dmg_done / 3;
        }

        //To log the dmg done
        enemy_hp - enemy.hp
    }

    pub fn trait_buffs(&mut self, enemy: &Warrior) {
        println!("\nHelmet trait: {:?}", self.helmet);
        match self.helmet {
            Trait::Iron => {
                if self.ar <= enemy.ap {
                    self.def += 2;
                    self.hp += 400;
                    println!("HP increased by 400!");
                } else {
                    println!("Trait didn't activate!");
                }
            }
            Trait::Skull => {
                if self.def <= enemy.atk {
                    self.def += 2;
                    self.hp += 400;
                    println!("HP increased by 400!");
                } else {
                    println!("Trait didn't activate!");
                }
            }
            Trait::Hood => {
                if self.atk <= enemy.def {
                    self.atk += 2;
                    self.damage += 50;
                    println!("Damage increased by 50!");
                } else {
                    println!("Trait didn't activate!");
                }
            }
            Trait::Raider => {
                if self.ap <= enemy.ar {
                    self.atk += 2;
                    self.damage += 50;
                    println!("Damage increased by 50!");
                } else {
                    println!("Trait didn't activate!");
                }
            }
            _ => {}
        }

        println!("Weapon trait: {:?}", self.weapon);
        match self.weapon {
            Trait::Halberd => {
                if self.atk >= enemy.atk {
                    self.atk += 2;
                    self.damage += 50;
                    println!("Damage increased by 50!");
                } else {
                    println!("Trait didn't activate!");
                }
            }
            Trait::Sword => {
                if self.ap >= enemy.ap {
                    self.ap += 2;
                    self.armor_pen += 200;
                    println!("Armor penetration increased by 200!");
                } else {
                    println!("Trait didn't activate!");
                }
            }
            Trait::Shield => {
                if self.def >= enemy.def {
                    self.def += 2;
                    self.hp += 400;
                    println!("HP increased by 400!");
                } else {
                    println!("Trait didn't activate!");
                }
            }
            Trait::Hammer => {
                if self.ar >= enemy.ar {
                    self.ar += 2;
                    self.armor += 200;
                    println!("Armor increased by 200!");
                } else {
                    println!("Trait didn't activate!");
                }
            }
            _ => {}
        }
    }

    pub fn background_check(
        &mut self,
        enemy: &mut Warrior,
        counter: u8,
        executing: &mut i32,
        died: &mut bool,
    ) {
        if !(enemy.background == Trait::Castle && (counter == 1 || counter == 10)) {
            println!("Attack for {:?}!", self.attack(enemy));
        }

        if self.background == Trait::Grassland && counter == 1 {
            println!(
                "Grassland trait activated! Attack for {:?}!",
                self.attack(enemy)
            );
            println!(
                "Grassland trait activated! Attack for {:?}!",
                self.attack(enemy)
            );
        }

        if self.background == Trait::Battlefield && enemy.hp <= (enemy.def as i32 * 150 + 1500) / 3
        {
            if *executing != 0 {
                enemy.hp -= *executing;
                println!("Execution!");
            } else {
                *executing = enemy.hp;
                println!("Gets ready to execute for {:?} damage!", enemy.hp);
            }
        }

        if enemy.background == Trait::Hell && enemy.hp <= 0 && !*died {
            println!("Dies, but the hell trait activates! He is revived at 1/4 of his HP!");
            enemy.hp = (enemy.def as i32 * 150 + 1500) / 4;
            *died = true;
        }
    }

    pub fn body_check(&mut self, counter: u8) {
        if self.body == Trait::Plate {
            //Increase armor by 3% each round
            self.armor = self.armor - (counter as i32 - 1) * 30 + counter as i32 * 30;
            println!("Armor: {:?}", self.armor);
        }

        if self.body == Trait::Leather {
            //Increase armor pen by 2% each round
            self.armor_pen = self.armor_pen - (counter as i32 - 1) * 20 + counter as i32 * 20;
            println!("Armor pen: {:?}", self.armor);
        }

        if self.body == Trait::Golden {
            if counter == 1 {
                //Gain 15% armor
                self.armor += 150;
            } else {
                //Lose 1.5% armor every round
                self.armor -= counter as i32 * 15;
            }
            println!("Armor: {:?}", self.armor);
        }

        if self.body == Trait::Demonic {
            if counter == 1 {
                //Gain 10% armor penetration
                self.armor_pen += 100;
            } else {
                //Lose 1% armor penetration each round
                self.armor_pen -= counter as i32 * 10;
            }
            println!("Armor pen: {:?}", self.armor);
        }
    }
}
