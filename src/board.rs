use std::collections::HashMap;

use crate::{relics::Relic, stats::Stats, tower::Tower};

#[derive(Clone)]
pub struct Board {
    pub towers: [Option<Tower>; 15],
    pub relics: HashMap<Relic, u32>,
    pub gold: u32,
}

impl Board {
    pub fn new() -> Board {
        Board { towers: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], relics: HashMap::new(), gold: 0 }
    }

    pub fn get_tower(&mut self, index: usize) -> Option<&mut Tower> {
        match self.towers.get_mut(index) {
            None => None, Some(v) => v.as_mut()
        }
    }

    pub fn set_tower(&mut self, index: usize, tower: Tower) {
        let kind= tower.kind.clone();
        self.towers[index] = Some(tower);
        kind.init(self, index);
    }
    pub fn destroy_tower(&mut self, index: usize) {
        self.towers[index] = None;
    }

    pub fn add_relic(&mut self, to_add: Relic) {
        *self.relics.entry(to_add).or_insert(0) += 1;
    }

    pub fn add_exp(&mut self, index: usize, amount: u32) {
        if let Some(tower) = self.towers.get_mut(index) {
            if let Some(tower) = tower.as_mut() {
                tower.add_exp(amount);
                tower.kind.on_receive_exp(amount, &mut self.gold);
            }
        }
    }

    pub fn add_stats(&mut self, index: usize, stats: Stats) {
        if let Some(v) =  self.get_tower(index) {
            v.stats += stats;
        }
    }

    pub fn add_temp_stats(&mut self, index: usize, mut stats: Stats) {
        let mut to_trigger = None;
        if let Some(v) = self.towers.get_mut(index) {
            if let Some(v) = v.as_mut() {
                for (relic, amount) in &self.relics {
                    for _ in 0..*amount {
                        stats = relic.on_temp_stats(stats);
                    }
                }
                v.temp_stats += stats;
                to_trigger = Some(v.kind.clone());
            }
        }
        if let Some(kind) = to_trigger {
            kind.on_receive_temp_stats(self, index);
        }
    }
}