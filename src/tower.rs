use std::collections::HashMap;

use crate::{board::Board, buffs::Buffs, stats::Stats, towers::{flower::Flower, honey::Honey, obsidian::Obsidian, spirit::Spirit}};

#[derive(Clone, Debug)]
pub struct Tower {
    pub stats: Stats,
    pub temp_stats: Stats,
    pub exp: u32,
    pub level: u32,
    pub exp_required: u32,
    pub base_stats: Stats,
    pub ability_uses: u32,
    pub buffs: HashMap<Buffs, u32>,
    pub kind: TowerType
}
impl Tower {
    pub fn new(kind: TowerType) -> Tower {
        let base_stats = kind.base_stats();
        Tower { stats: base_stats, temp_stats: Stats::new(0.0, 0.0), exp: 0, level: 1, exp_required: 10, base_stats, ability_uses: 0, buffs: HashMap::new(), kind }
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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum TowerType {
    Spirit,
    Flower,
    Honey,
    Obsidian
}
impl TowerType {
    pub fn rarity(&self) -> TowerRarity {
        match self {
            TowerType::Spirit => TowerRarity::Rare,
            TowerType::Flower | TowerType::Obsidian => TowerRarity::Uncommon,
            TowerType::Honey => TowerRarity::Common
        }
    }

    pub fn base_stats(&self) -> Stats {
        match self {
            TowerType::Spirit => Stats::new(50.0, 50.0),
            TowerType::Flower => Stats::new(5.0, 5.0),
            TowerType::Obsidian => Stats::new(3.0, 3.0),
            TowerType::Honey => Stats::new(1.0, 2.0)
        }
    }

    pub fn init(&self, board: &mut Board, index: usize) {
        match self {
            TowerType::Spirit => Spirit::init(board, index),
            _ => {}
        }
    }

    pub fn round_start(&self, board: &mut Board, index: usize) {
        match self {
            TowerType::Spirit => Spirit::round_start(board, index),
            _ => {}
        }
    }

    pub fn round_end(&self, board: &mut Board, index: usize) {
        match self {
            TowerType::Flower => Flower::round_end(board, index),
            _ => {}
        }
    }

    pub fn round_end_self_destruct(&self, board: &mut Board, index: usize) {
        match self {
            TowerType::Honey => Honey::round_end_self_destruct(board, index),
            _ => {}
        }
    }

    pub fn use_ability(&self, board: &mut Board, index: usize, on: usize) {
        match self {
            TowerType::Spirit => Spirit::use_ability(board, index, on),
            _ => {}
        }
    }

    pub fn on_receive_exp(&self, amount: u32, gold: &mut u32) {
        match self {
            TowerType::Flower => Flower::on_receive_exp(amount, gold),
            _ => {}
        }
    }

    pub fn on_receive_temp_stats(&self, board: &mut Board, index: usize) {
        match self {
            TowerType::Obsidian => Obsidian::on_receive_temp_stats(board, index),
            _ => {}
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum TowerRarity {
    Common,
    Uncommon,
    Rare
}