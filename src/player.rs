use crate::{
    class::{Class, Rogue},
    fight::Fight,
};
use roll_dice::roll;
use std::collections::HashMap;

pub type DamageModifiers = HashMap<usize, Vec<Box<dyn Fn(usize) -> usize>>>;

pub struct Player {
    pub health: usize,
    pub class: Box<dyn Class>,
    pub damage_modifiers: DamageModifiers,
}

impl Player {
    pub fn attack(&mut self, rival: &mut Player, tick: usize) -> () {
        let min_roll = self.class.min_roll();

        let roll = roll_dices();

        if roll >= min_roll {
            let damage = (roll as isize - min_roll as isize) + self.class.attack_bonus()
                - rival.class.defense_bonus();

            let atk_damage = self.class.attack_modifier(
                damage as usize,
                &mut self.damage_modifiers,
                &mut rival.damage_modifiers,
            );
            let mut final_damage = rival.class.defend_modifier(
                atk_damage,
                &mut rival.damage_modifiers,
                &mut self.damage_modifiers,
            );

            if self.damage_modifiers.contains_key(&tick) {
                for modifier in self.damage_modifiers.get(&tick).unwrap() {
                    final_damage = modifier(final_damage);
                }
            }

            if rival.health >= final_damage {
                rival.health -= final_damage as usize;
            } else {
                rival.health = 0;
            }
        }

        ()
    }
}

fn roll_dices() -> usize {
    let dices: Vec<u32> = roll(2, 6);
    dices.iter().fold(0, |a, b| a + b.to_owned() as usize)
}
