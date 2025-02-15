use crate::tower_type::{Initializable, ReactingToLevelup};

pub struct Ladder {}

impl Initializable for Ladder {
    fn init(board: &mut crate::board::Board, index: usize) {
        board.get_tower(index).unwrap().exp_required = 5;
    }
}
impl ReactingToLevelup for Ladder {
    fn on_levelup(tower: &mut crate::tower::Tower) {
        tower.exp_required = 5;
    }
}