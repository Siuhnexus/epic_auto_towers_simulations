use crate::board::Board;

pub trait Strategy {
    fn init(self, board: &mut Board, gold: &mut u32, round: &mut u32);
    fn round_actions(self, board: &mut Board, gold: &mut u32);
    fn last_round(&self) -> u32;
}