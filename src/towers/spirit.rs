use crate::{buffs::Buffs, stats::Stats, tower::{Board, TowerType}};

#[derive(Clone)]
pub struct Spirit {}

impl TowerType for Spirit {
    fn base_stats(&self) -> Stats {
        Stats::new(1.0, 1.0)
    }

    fn init(self, towers: &mut Board, index: usize) {
        let tower = towers.get_mut(index).unwrap().as_mut().unwrap();
        tower.add_exp(4);
    }

    fn round_start(self, towers: &mut Board, index: usize) {
        let tower = towers.get_mut(index).unwrap().as_mut().unwrap();
        tower.add_exp(2);
        tower.ability_uses = tower.level;
    }

    fn round_end(self, towers: &mut Board, index: usize) {
        todo!()
    }

    fn round_end_self_destruct(self, towers: Board, index: usize) {
        todo!()
    }

    fn use_ability(mut self, towers: &mut Board, index: usize, on: usize) {
        let tower = towers.get_mut(index).unwrap().as_mut().unwrap();
        if tower.ability_uses <= 0 { return; }
        tower.ability_uses -= 1;

        let target = towers.get_mut(on).unwrap().as_mut().unwrap();
        *target.buffs.entry(Buffs::Chakra).or_insert(0) += 1;
    }
}