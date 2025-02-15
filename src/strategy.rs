use crate::{board::Board, logger::CSVLogger};

pub trait Strategy {
    fn init(&self, board: &mut Board, round: &mut u32);
    fn round_actions(&self, board: &mut Board, round: u32);
    fn log_init(&self, logger: &mut CSVLogger);
    fn log(&self, logger: &mut CSVLogger, board: &Board, round: u32);
    fn last_round(&self) -> u32;
}