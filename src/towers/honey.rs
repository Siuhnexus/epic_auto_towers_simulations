use crate::board::Board;

pub struct Honey {}
impl Honey {
    pub fn round_end_self_destruct(board: &mut Board, index: usize) {
        let tower = board.get_tower(index).unwrap();
        let stats = tower.stats + tower.temp_stats;
        board.add_stats(index - 1, stats);
        board.destroy_tower(index);
    }
}