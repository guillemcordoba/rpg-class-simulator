use crate::player::DamageModifiers;

pub trait AttackSkill {
    fn min_roll(&self) -> usize;
    fn attack_bonus(&self) -> isize;

    fn attack(
        &self,
        damage: usize,
        tick: usize,
        first: bool,
        self_damage_modifiers: &mut DamageModifiers,
        rival_damage_modifiers: &mut DamageModifiers,
    ) -> usize {
        damage
    }
}

pub trait DefenseSkill {
    fn defense_bonus(&self) -> isize {
        0
    }
    fn defense_roll_bonus(&self) -> isize {
        0
    }
    fn defend(
        &self,
        damage: usize,
        tick: usize,
        first: bool,
        self_damage_modifiers: &mut DamageModifiers,
        rival_damage_modifiers: &mut DamageModifiers,
    ) -> usize {
        damage
    }
}
