use crate::player::DamageModifiers;
use crate::{
    class::Class,
    skill::{AttackSkill, DefenseSkill},
};

pub struct HammerGuy;

impl Class for HammerGuy {
    fn name(&self) -> String {
        String::from("HammerGuy")
    }

    fn choose_attack_skill(
        &self,
        _self_health: usize,
        rival_health: usize,
    ) -> Box<dyn AttackSkill> {
        if rival_health < 4 {
            Box::new(HammerSweep)
        } else {
            self.basic_attack_skill()
        }
    }

    fn basic_attack_skill(&self) -> Box<dyn AttackSkill> {
        Box::new(Hammer)
    }
    fn basic_defense_skill(&self) -> Box<dyn DefenseSkill> {
        Box::new(MetalArmor)
    }
}

pub struct Hammer;

impl AttackSkill for Hammer {
    fn min_roll(&self) -> usize {
        8
    }
    fn attack_bonus(&self) -> isize {
        8
    }
}

pub struct HammerSweep;

impl AttackSkill for HammerSweep {
    fn min_roll(&self) -> usize {
        4
    }
    fn attack_bonus(&self) -> isize {
        6
    }

    fn attack(
        &self,
        damage: usize,
        tick: usize,
        first: bool,
        self_damage_modifiers: &mut DamageModifiers,
        rival_damage_modifiers: &mut DamageModifiers,
    ) -> usize {
        let miss = |_d: usize| 0;

        self_damage_modifiers.insert(tick + 1, vec![Box::new(miss)]);

        if first {
            rival_damage_modifiers.insert(tick, vec![Box::new(|d| d + 6)]);
        }

        damage
    }
}

pub struct MetalArmor;

impl DefenseSkill for MetalArmor {
    fn defense_bonus(&self) -> isize {
        1
    }
}
