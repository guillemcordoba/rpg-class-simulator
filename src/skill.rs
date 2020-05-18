use crate::player::DamageModifiers;

pub trait AttackSkill {
    fn min_roll(&self) -> usize;
    fn attack_bonus(&self) -> isize;

    fn attack(
        &self,
        damage: usize,
        _tick: usize,
        _first: bool,
        _self_damage_modifiers: &mut DamageModifiers,
        _rival_damage_modifiers: &mut DamageModifiers,
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
        _tick: usize,
        _first: bool,
        _self_damage_modifiers: &mut DamageModifiers,
        _rival_damage_modifiers: &mut DamageModifiers,
    ) -> usize {
        damage
    }
}
