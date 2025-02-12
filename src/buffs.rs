use crate::{board::Board, stats::Stats};

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub enum Buffs {
    Chakra
}

impl Buffs {
    pub fn round_end(&self, board: &mut Board, index: usize, amount: u32) {
        match self {
            Buffs::Chakra => {
                board.add_exp(index, amount);
                board.add_stats(index, Stats::new(2.0, 3.0) * amount);
            }
        }
    }
}