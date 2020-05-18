use crate::{
    class::{Class, Knight, Rogue},
    matchup::Matchup,
};

pub type ClassGenerator = Box<dyn Fn() -> Box<dyn Class>>;

pub fn generate_matchups(
    fight_number: usize,
    classes_generators: Vec<ClassGenerator>,
) -> Vec<Matchup> {
    let mut matchups: Vec<Matchup> = Vec::new();

    for (i, class_i) in classes_generators.iter().enumerate() {
        for (j, class_j) in classes_generators.iter().enumerate() {
            if i > j {
                let matchup = Matchup::from_generators(fight_number, &class_i, &class_j);

                matchups.push(matchup);
            }
        }
    }

    matchups
}

pub fn classes_generators() -> Vec<ClassGenerator> {
    vec![
        Box::new(|| (Box::new(Rogue) as Box<dyn Class>)),
        Box::new(|| (Box::new(Knight) as Box<dyn Class>)),
    ]
}
