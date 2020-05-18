use crate::{class::Class, fight::Fight, player::Player, utils};
use std::collections::HashMap;

pub struct Matchup {
    first_class: Box<dyn Class>,
    second_class: Box<dyn Class>,
    fights: Vec<Fight>,
    first_wins: usize,
    second_wins: usize,
    pub attacker_wins: usize,
    pub defender_wins: usize,
}

impl Matchup {
    pub fn from_generators(
        fight_number: usize,
        first: &Box<dyn Fn() -> Box<dyn Class>>,
        second: &Box<dyn Fn() -> Box<dyn Class>>,
    ) -> Matchup {
        let mut fights: Vec<Fight> = Vec::new();
        for _x in 0..fight_number {
            let first_class = first();
            let first_player = Player {
                health: first_class.initial_health(),
                class: first_class,
                damage_modifiers: HashMap::new(),
            };
            let second_class = second();
            let second_player = Player {
                health: second_class.initial_health(),
                class: second_class,
                damage_modifiers: HashMap::new(),
            };

            let fight = Fight::new(first_player, second_player);

            fights.push(fight);
        }

        Matchup {
            fights,
            first_class: first(),
            second_class: second(),
            first_wins: 0,
            second_wins: 0,
            attacker_wins: 0,
            defender_wins: 0,
        }
    }

    pub fn simulate(&mut self) -> () {
        for fight in self.fights.iter_mut() {
            let (winner, _) = fight.simulate();

            if winner.class.name() == self.first_class.name() {
                self.first_wins += 1;
            } else {
                self.second_wins += 1;
            }

            if fight.attacker_won().unwrap() {
                self.attacker_wins += 1;
            } else {
                self.defender_wins += 1;
            }
        }

        ()
    }

    pub fn result(&self) -> String {
        utils::winrate(
            self.first_class.name(),
            self.second_class.name(),
            self.first_wins,
            self.second_wins,
        )
    }
}
