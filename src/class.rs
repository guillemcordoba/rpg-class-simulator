use crate::skill::{AttackSkill, DefenseSkill};

pub trait Class {
    fn name(&self) -> String;

    fn initial_health(&self) -> usize {
        30
    }

    fn choose_attack_skill(
        &self,
        _self_health: usize,
        _rival_health: usize,
    ) -> Box<dyn AttackSkill> {
        self.basic_attack_skill()
    }

    fn choose_defense_skill(
        &self,
        _self_health: usize,
        _rival_health: usize,
    ) -> Box<dyn DefenseSkill> {
        self.basic_defense_skill()
    }

    fn basic_attack_skill(&self) -> Box<dyn AttackSkill>;
    fn basic_defense_skill(&self) -> Box<dyn DefenseSkill>;
}
