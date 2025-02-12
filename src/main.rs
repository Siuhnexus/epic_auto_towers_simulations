use epic_auto_towers_simulations::{simulator::Simulator, strategies::spirit_max_level::SpiritMaxLevel};

fn main() {
    let sim = Simulator::new();
    sim.simulate(SpiritMaxLevel {});
}
