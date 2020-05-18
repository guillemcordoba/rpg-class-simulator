extern crate roll_dice;

mod class;
mod fight;
mod generators;
mod matchup;
mod player;
mod utils;
mod classes;
mod skill;

fn main() {
    let classes_generators = generators::classes_generators();
    let fights = generators::generate_matchups(10000, classes_generators);

    let mut attacker_wins = 0;
    let mut defender_wins = 0;

    for mut matchup in fights.into_iter() {
        matchup.simulate();
        println!("{}", matchup.result());

        attacker_wins += matchup.attacker_wins;
        defender_wins += matchup.defender_wins;
    }

    println!("");

    let winrate = utils::winrate(
        String::from("First Attacker"),
        String::from("First Defender"),
        attacker_wins,
        defender_wins,
    );

    println!("Global order winrate: {}", winrate);
}
