use crate::{board::Board, tower_type::{Initializable, RoundStarting}};

pub struct Debt {}

impl Initializable for Debt {
    fn init(board: &mut Board, index: usize) {
        let tower = board.get_tower(index).unwrap();
        tower.x_value = -6;
    }
}
impl RoundStarting for Debt {
    fn round_start(board: &mut Board, index: usize) {
        let tower = board.get_tower(index).unwrap();
        let gold = tower.x_value;
        tower.x_value += 1;
        board.gold = (board.gold as i32 + gold).max(0) as u32;
    }
}