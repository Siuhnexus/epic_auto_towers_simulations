use crate::{board::Board, buffs::Buffs, stats::Stats, tower::TowerType};

#[derive(Clone, Debug)]
pub struct Spirit {}

impl TowerType for Spirit {
    fn base_stats(&self) -> Stats {
        Stats::new(50.0, 50.0)
    }

    fn init(&self, board: &mut Board, index: usize) {
        let tower = board.get_tower(index).unwrap();
        tower.add_exp(6);
        tower.ability_uses = 1;
    }

    fn round_start(&self, board: &mut Board, index: usize) {
        let tower = board.get_tower(index).unwrap();
        tower.add_exp(2);
        tower.ability_uses = tower.level;
    }

    fn round_end(&self, _board: &mut Board, _index: usize) {
        
    }

    fn round_end_self_destruct(&self, _board: &mut Board, _index: usize) {
        
    }

    fn use_ability(&self, board: &mut Board, index: usize, on: usize) {
        let tower = board.get_tower(index).unwrap();
        if tower.ability_uses <= 0 { return; }
        tower.ability_uses -= 1;

        let target = board.get_tower(on).unwrap();
        *target.buffs.entry(Buffs::Chakra).or_insert(0) += 1;
    }
}