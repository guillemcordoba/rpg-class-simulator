use crate::player::DamageModifiers;
use std::fmt::Debug;

pub trait Class: Debug {
    fn name(&self) -> String;

    fn initial_health(&self) -> usize {
        30
    }
    fn min_roll(&self) -> usize;
    fn attack_bonus(&self) -> isize;
    fn defense_bonus(&self) -> isize {
        0
    }

    fn attack_modifier(
        &self,
        damage: usize,
        _self_damage_modifiers: &mut DamageModifiers,
        _rival_damage_modifiers: &mut DamageModifiers,
    ) -> usize {
        damage
    }
    fn defend_modifier(
        &self,
        damage: usize,
        _self_damage_modifiers: &mut DamageModifiers,
        _rival_damage_modifiers: &mut DamageModifiers,
    ) -> usize {
        damage
    }
}

#[derive(Debug)]
pub struct Rogue;

impl Class for Rogue {
    fn name(&self) -> String {
        String::from("Rogue")
    }

    fn min_roll(&self) -> usize {
        3
    }
    fn attack_bonus(&self) -> isize {
        0
    }
}

#[derive(Debug)]
pub struct Knight;

impl Class for Knight {
    fn name(&self) -> String {
        String::from("Knight")
    }

    fn min_roll(&self) -> usize {
        8
    }
    fn attack_bonus(&self) -> isize {
        8
    }
}
