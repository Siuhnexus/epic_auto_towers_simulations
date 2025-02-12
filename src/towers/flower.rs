use crate::{board::Board, tower_type::{ReactingToEXP, RoundEnding}};

pub struct Flower {}

impl ReactingToEXP for Flower {
    fn on_receive_exp(amount: u32, gold: &mut u32) {
        *gold += amount;
    }
}
impl RoundEnding for Flower {
    fn round_end(board: &mut Board, index: usize) {
        let stats = board.get_tower(index).unwrap().stats;
        board.add_temp_stats(index - 1, stats * 0.2);
        board.add_temp_stats(index + 1, stats * 0.2);
    }
}