use crate::{
    class::Class,
    skill::{AttackSkill, DefenseSkill},
};

pub struct HeavyKnight;

impl Class for HeavyKnight {
    fn name(&self) -> String {
        String::from("HeavyKnight")
    }

    fn basic_attack_skill(&self) -> Box<dyn AttackSkill> {
        Box::new(Claymore)
    }

    fn basic_defense_skill(&self) -> Box<dyn DefenseSkill> {
        Box::new(TankActitude)
    }
}

pub struct Claymore;

impl AttackSkill for Claymore {
    fn min_roll(&self) -> usize {
        7
    }
    fn attack_bonus(&self) -> isize {
        5
    }
}

pub struct TankActitude;

impl DefenseSkill for TankActitude {
    fn defense_bonus(&self) -> isize {
        3
    }

    fn defense_roll_bonus(&self) -> isize {
        1
    }
}
