use crate::class::Class;
use roll_dice::roll;
use std::collections::HashMap;

pub type DamageModifiers = HashMap<usize, Vec<Box<dyn Fn(usize) -> usize>>>;

pub struct Player {
    pub health: usize,
    pub class: Box<dyn Class>,
    pub damage_modifiers: DamageModifiers,
}

impl Player {
    pub fn attack(&mut self, rival: &mut Player, tick: usize, first: bool) -> () {
        let attacking_skill = self.class.choose_attack_skill(self.health, rival.health);
        let defending_skill = rival.class.choose_defense_skill(rival.health, self.health);

        let min_roll = attacking_skill.min_roll();

        let mut roll: isize = roll_dices() as isize;

        roll += defending_skill.defense_roll_bonus();

        if roll < 0 {
            roll = 0;
        }

        if roll as usize >= min_roll {
            let mut damage = (roll as isize - min_roll as isize) + attacking_skill.attack_bonus()
                - defending_skill.defense_bonus();

            if damage < 0 {
                damage = 0;
            }

            let atk_damage = attacking_skill.attack(
                damage as usize,
                tick,
                first,
                &mut self.damage_modifiers,
                &mut rival.damage_modifiers,
            );
            let mut final_damage = defending_skill.defend(
                atk_damage,
                tick,
                !first,
                &mut self.damage_modifiers,
                &mut rival.damage_modifiers,
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
