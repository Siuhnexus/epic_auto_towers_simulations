use epic_auto_towers_simulations::{simulator::Simulator, strategies::princess_test::PrincessTest};

fn main() {
    let sim = Simulator::new();
    sim.simulate(PrincessTest {});
}
