use epic_auto_towers_simulations::{stats::Stats, tower::Tower};

fn step_flower(flower: &mut Tower, obsidian: &mut Tower, chakra: u32, chakra_stats: &Stats, gold: &mut u32) {
    // Manual actions in round
    let new_obsidians = *gold / 2 / 5;
    obsidian.add_exp(new_obsidians * 5);
    *gold = 0;
    flower.chakra += chakra;
    // End of round effects
    let mut honey_stats = flower.stats * 0.3; // Flower gives 20% which is increased by 50% through hourglass
    obsidian.stats += honey_stats * (0.5 + obsidian.level as f64 * 0.25).max(1.0);
    honey_stats += Stats::new(1.0, 2.0); // Honey has (1, 2) starting stats
    flower.add_exp(flower.chakra);
    *gold += flower.chakra;
    flower.stats += flower.chakra * chakra_stats;
    flower.stats += honey_stats;
}
fn step_obsidian(flower: &mut Tower, obsidian: &mut Tower, chakra: u32, chakra_stats: &Stats, gold: &mut u32) {
    // Manual actions in round
    let new_obsidians = *gold / 2 / 5;
    obsidian.add_exp(new_obsidians * 5);
    *gold = 0;
    obsidian.chakra += chakra;
    // End of round effects
    let mut honey_stats = flower.stats * 0.3; // Flower gives 20% which is increased by 50% through hourglass
    obsidian.stats += honey_stats * (0.5 + obsidian.level as f64 * 0.25).max(1.0);
    honey_stats += Stats::new(1.0, 2.0); // Honey has (1, 2) starting stats
    obsidian.add_exp(obsidian.chakra);
    obsidian.stats += obsidian.chakra * chakra_stats * (0.5 + obsidian.level as f64 * 0.25).max(1.0);
    flower.stats += honey_stats;
}

fn test_conditions(mut starting_round: u32, ending_round: u32, chakra_per_round: u32, starting_flower: &Tower, starting_obsidian: &Tower) {
    let chakra_stats = Stats::new(2.0, 3.0);

    let mut flower1 = starting_flower.clone();
    let mut flower2 = starting_flower.clone();
    let mut obsidian1 = starting_obsidian.clone();
    let mut obsidian2 = starting_obsidian.clone();

    let mut gold_flower = 0;

    while starting_round <= ending_round {
        step_obsidian(&mut flower1, &mut obsidian1, chakra_per_round, &chakra_stats, &mut 0);
        step_flower(&mut flower2, &mut obsidian2, chakra_per_round, &chakra_stats, &mut gold_flower);
        starting_round += 1;
    }
    println!("{chakra_per_round} Chakra: {} Chakra auf Obsidian ({}) vs. {} Chakra auf Blume ({})", obsidian1.stats, obsidian1.level, obsidian2.stats, obsidian2.level);
}

fn main() {
    let starting_round = 22;
    let ending_round = 68;
    let flower = Tower::new(237.0, 301.0, 30, Stats::new(5.0, 5.0));
    let obsidian = Tower::new(271.0, 338.0, 9, Stats::new(3.0, 3.0));

    for chakra in 1..=15 {
        test_conditions(starting_round, ending_round, chakra, &flower, &obsidian);
    }
}
