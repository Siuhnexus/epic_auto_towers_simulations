use crate::{board::Board, tower_type::SelfDestructing};

pub struct Honey {}

impl SelfDestructing for Honey {
    fn round_end_self_destruct(board: &mut Board, index: usize) {
        let tower = board.get_tower(index).unwrap();
        let stats = tower.stats + tower.temp_stats;
        board.add_stats(index - 1, stats);
        board.destroy_tower(index);
    }
}