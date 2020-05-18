use crate::player::DamageModifiers;
use crate::{
    class::Class,
    skill::{AttackSkill, DefenseSkill},
};
use roll_dice::roll;

pub struct Gangster;

impl Class for Gangster {
    fn name(&self) -> String {
        String::from("Gangster")
    }

    fn basic_attack_skill(&self) -> Box<dyn AttackSkill> {
        Box::new(Knucles)
    }

    fn basic_defense_skill(&self) -> Box<dyn DefenseSkill> {
        Box::new(LeatherArmor)
    }
}

pub struct Knucles;

impl AttackSkill for Knucles {
    fn min_roll(&self) -> usize {
        6
    }
    fn attack_bonus(&self) -> isize {
        6
    }
}

pub struct LeatherArmor;

impl DefenseSkill for LeatherArmor {
    fn defense_bonus(&self) -> isize {
        -2
    }

    fn defend(
        &self,
        damage: usize,
        tick: usize,
        first: bool,
        self_damage_modifiers: &mut DamageModifiers,
        rival_damage_modifiers: &mut DamageModifiers,
    ) -> usize {
        let dice = roll(1, 6);

        if dice[0] == 6 {
            let final_damage: isize = damage as isize - 8;
            if final_damage > 0 {
                final_damage as usize
            } else {
                0
            }
        } else {
            damage
        }
    }
}
