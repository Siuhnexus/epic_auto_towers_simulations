use crate::board::Board;

pub struct Flower {}
impl Flower {
    pub fn on_receive_exp(amount: u32, gold: &mut u32) {
        *gold += amount;
    }

    pub fn round_end(board: &mut Board, index: usize) {
        let stats = board.get_tower(index).unwrap().stats;
        board.add_temp_stats(index - 1, stats * 0.2);
        board.add_temp_stats(index + 1, stats * 0.2);
    }
}