use crate::board::Board;

pub trait Strategy {
    fn init(&self, board: &mut Board, round: &mut u32);
    fn round_actions(&self, board: &mut Board, round: u32);
    fn log_init(&self) -> String;
    fn log(&self, board: &Board, round: u32) -> String;
    fn last_round(&self) -> u32;
}