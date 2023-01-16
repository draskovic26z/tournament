use crate::state::Warrior;

pub fn commence_battle(p1: &mut Warrior, p2: &mut Warrior) {
    let mut counter: u8 = 1;
    //Battlefield helper variables
    let mut executing1 = 0;
    let mut executing2 = 0;
    //Hell helper vars
    let mut died1 = false;
    let mut died2 = false;
    loop {
        if counter == 11_u8 || p1.hp <= 0 || p2.hp <= 0 {
            break;
        }
        println!("\nRound: {:?}", counter);

        println!("P1:");
        p1.body_check(counter);
        println!("P2:");
        p2.body_check(counter);

        //This does the attack also
        println!("\nP1:");
        p1.background_check(p2, counter, &mut executing2, &mut died2);
        println!("P2:");
        p2.background_check(p1, counter, &mut executing1, &mut died1);

        println!("\nP1 Health: {:?}", p1.hp);
        println!("P2 Health: {:?}", p2.hp);

        counter += 1;
    }
    if p1.hp > p2.hp {
        p1.won = true;
    } else {
        p2.won = true;
    }
}
