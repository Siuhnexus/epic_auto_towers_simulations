use std::collections::HashMap;

use dyn_clone::DynClone;

use crate::{board::Board, buffs::Buffs, stats::Stats};

#[derive(Clone, Debug)]
pub struct Tower {
    pub stats: Stats,
    pub exp: u32,
    pub level: u32,
    pub exp_required: u32,
    pub base_stats: Stats,
    pub ability_uses: u32,
    pub buffs: HashMap<Buffs, u32>,
    pub kind: Box<dyn TowerType>
}

impl Tower {
    pub fn new(kind: Box<dyn TowerType>) -> Tower {
        let base_stats = kind.base_stats();
        Tower { stats: base_stats, exp: 0, level: 1, exp_required: 10, base_stats, ability_uses: 0, buffs: HashMap::new(), kind }
    }

    pub fn add_exp(&mut self, gained: u32) {
        self.exp += gained;
        while self.exp >= self.exp_required {
            self.level += 1;
            self.stats += self.base_stats;
            self.exp -= self.exp_required;
            self.exp_required += 5;
        }
    }
}

pub trait TowerType: DynClone + std::fmt::Debug {
    fn base_stats(&self) -> Stats;
    fn init(&self, board: &mut Board, index: usize);
    fn round_start(&self, board: &mut Board, index: usize);
    fn round_end(&self, board: &mut Board, index: usize);
    fn round_end_self_destruct(&self, board: &mut Board, index: usize);
    fn use_ability(&self, board: &mut Board, index: usize, on: usize);
}
dyn_clone::clone_trait_object!(TowerType);