use epic_auto_towers_simulations::{simulator::Simulator, strategy::Strategy, tower::Tower, towers::spirit::Spirit};

struct SpiritMaxLevel {}
impl Strategy for SpiritMaxLevel {
    fn init(self, board: &mut epic_auto_towers_simulations::board::Board, gold: &mut u32, round: &mut u32) {
        *round = 22;
        board.towers[0] = Some(Tower::new(Box::new(Spirit {})));
    }

    fn round_actions(self, board: &mut epic_auto_towers_simulations::board::Board, gold: &mut u32) {
        todo!()
    }

    fn last_round(&self) -> u32 {
        106
    }
}

fn main() {
    let sim = Simulator::new();
    sim.simulate(SpiritMaxLevel {});
}
