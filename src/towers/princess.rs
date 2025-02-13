use crate::tower_type::RoundStarting;

pub struct Princess {}

impl RoundStarting for Princess {
    fn round_start(board: &mut crate::board::Board, index: usize) {
        let exp = board.get_tower(index).unwrap().level * 2;
        board.add_exp(index - 1, exp);
        board.add_exp(index + 1, exp);
    }
}