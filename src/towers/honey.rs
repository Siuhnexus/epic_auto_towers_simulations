use crate::board::Board;

#[derive(Debug, Clone)]
pub struct Honey {}
impl Honey {
    fn round_end_self_destruct(&self, board: &mut Board, index: usize) {
        let tower = board.get_tower(index).unwrap();
        let stats = tower.stats + tower.temp_stats;
        board.add_stats(index - 1, stats);
        board.destroy_tower(index);
    }
}