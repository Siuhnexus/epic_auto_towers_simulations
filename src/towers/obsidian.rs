use crate::{board::Board, stats::Stats};

pub struct Obsidian {}
impl Obsidian {
    pub fn on_receive_temp_stats(board: &mut Board, index: usize) {
        let tower = board.get_tower(index).unwrap();
        tower.stats += tower.temp_stats * (0.5 * tower.level as f64).max(1.0);
        tower.temp_stats = Stats::new(0.0, 0.0);
    }
}