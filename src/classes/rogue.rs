use crate::player::DamageModifiers;
use crate::{
    class::Class,
    skill::{AttackSkill, DefenseSkill},
};
use roll_dice::roll;

pub struct Rogue;

impl Class for Rogue {
    fn name(&self) -> String {
        String::from("Rogue")
    }

    fn basic_attack_skill(&self) -> Box<dyn AttackSkill> {
        Box::new(Dagger)
    }

    fn basic_defense_skill(&self) -> Box<dyn DefenseSkill> {
        Box::new(AssassinActitude)
    }
}

pub struct Dagger;

impl AttackSkill for Dagger {
    fn min_roll(&self) -> usize {
        3
    }
    fn attack_bonus(&self) -> isize {
        0
    }
}

pub struct AssassinActitude;

impl DefenseSkill for AssassinActitude {
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
        let dice = roll(2, 6);

        if dice[0] + dice[1] > 7 {
            0
        } else {
            damage
        }
    }
}
