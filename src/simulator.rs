use crate::{board::Board, strategy::Strategy};

pub struct Simulator {}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator {}
    }

    pub fn simulate(self, strategy: impl Strategy) {
        let mut gold = 0;
        let mut round = 1;
        let mut board = Board::new();

        strategy.init(&mut board, &mut gold, &mut round);
    }
}