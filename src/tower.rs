use std::collections::HashMap;

use crate::{buffs::Buffs, stats::Stats, tower_type::{ReactingToLevelup, TowerType}, towers::ladder::Ladder};

#[derive(Clone, Debug)]
pub struct Tower {
    pub stats: Stats,
    pub temp_stats: Stats,
    pub exp: u32,
    pub level: u32,
    pub exp_required: u32,
    pub base_stats: Stats,
    pub ability_uses: u32,
    pub x_value: i32,
    pub buffs: HashMap<Buffs, u32>,
    pub kind: TowerType
}
impl Tower {
    pub fn new(kind: TowerType) -> Tower {
        let base_stats = kind.base_stats();
        Tower { stats: base_stats, temp_stats: Stats::new(0.0, 0.0), exp: 0, level: 1, exp_required: 10, base_stats, ability_uses: 0, x_value: 0, buffs: HashMap::new(), kind }
    }

    pub fn add_exp(&mut self, gained: u32) {
        self.exp += gained;
        while self.exp >= self.exp_required {
            self.level += 1;
            self.stats += self.base_stats;
            self.exp -= self.exp_required;
            self.exp_required += 5;
            match self.kind {
                TowerType::Ladder => {
                    Ladder::on_levelup(self);
                }, _ => {}
            }
        }
    }
}