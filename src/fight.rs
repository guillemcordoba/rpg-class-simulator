use crate::player::Player;
use roll_dice::roll;

pub struct Fight {
    first: Player,
    second: Player,
    tick: usize,
}

impl Fight {
    pub fn new(player_a: Player, player_b: Player) -> Fight {
        let dices = roll(1, 2);

        if dices[0] == 1 {
            Fight {
                first: player_a,
                second: player_b,
                tick: 0,
            }
        } else {
            Fight {
                first: player_b,
                second: player_a,
                tick: 0,
            }
        }
    }

    pub fn simulate(&mut self) -> (&Player, &Player) {
        while let None = self.winner() {
            self.tick();
        }

        (self.winner().unwrap(), self.loser().unwrap())
    }

    pub fn tick(&mut self) -> () {
        self.first.attack(&mut self.second, self.tick, true);

        if let None = self.winner() {
            self.second.attack(&mut self.first, self.tick, false);
        }

        self.tick += 1;

        ()
    }

    pub fn winner(&self) -> Option<&Player> {
        if self.first.health == 0 {
            return Some(&self.second);
        } else if self.second.health == 0 {
            return Some(&self.first);
        }
        None
    }

    pub fn loser(&self) -> Option<&Player> {
        if self.first.health == 0 {
            return Some(&self.first);
        } else if self.second.health == 0 {
            return Some(&self.second);
        }
        None
    }

    pub fn attacker_won(&self) -> Option<bool> {
        let winner = self.winner();

        winner.map(|w| w.class.name() == self.first.class.name())
    }
}
