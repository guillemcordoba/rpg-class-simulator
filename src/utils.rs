pub fn winrate(player_a: String, player_b: String, wins_a: usize, wins_b: usize) -> String {
    if wins_a == wins_b {
        format!("{} and {} are tied", player_a, player_b,)
    } else if wins_a > wins_b {
        let winrate: f64 = 100.0 * wins_a as f64 / (wins_a + wins_b) as f64;
        format!(
            "{} wins {} with a winrate of {}%",
            player_a, player_b, winrate,
        )
    } else {
        let winrate: f64 = 100.0 * (wins_b as f64 / (wins_a + wins_b) as f64);
        format!(
            "{} wins {} with a winrate of {}%",
            player_b, player_a, winrate,
        )
    }
}
