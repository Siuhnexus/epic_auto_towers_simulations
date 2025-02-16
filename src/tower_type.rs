use crate::{board::Board, stats::Stats, tower::Tower, towers::{debt::Debt, flower::Flower, honey::Honey, ladder::Ladder, obsidian::Obsidian, princess::Princess, spirit::Spirit}};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum TowerType {
    Spirit,
    Flower,
    Honey,
    Obsidian,
    Princess,
    Ladder,
    Debt
}
impl TowerType {
    pub fn rarity(&self) -> TowerRarity {
        match self {
            TowerType::Spirit | TowerType::Princess => TowerRarity::Rare,
            TowerType::Flower | TowerType::Obsidian => TowerRarity::Uncommon,
            TowerType::Honey | TowerType::Ladder => TowerRarity::Common,
            TowerType::Debt => TowerRarity::Malicious
        }
    }

    pub fn base_stats(&self) -> Stats {
        match self {
            TowerType::Spirit => Stats::new(50.0, 50.0),
            TowerType::Flower => Stats::new(5.0, 5.0),
            TowerType::Obsidian | TowerType::Princess | TowerType::Ladder => Stats::new(3.0, 3.0),
            TowerType::Honey => Stats::new(1.0, 2.0),
            TowerType::Debt => Stats::new(1.0, 5.0)
        }
    }

    pub fn init(&self, board: &mut Board, index: usize) {
        match self {
            TowerType::Spirit => Spirit::init(board, index),
            TowerType::Ladder => Ladder::init(board, index),
            TowerType::Debt => Debt::init(board, index),
            _ => {}
        }
    }

    pub fn round_start(&self, board: &mut Board, index: usize) {
        match self {
            TowerType::Spirit => Spirit::round_start(board, index),
            TowerType::Princess => Princess::round_start(board, index),
            TowerType::Debt => Debt::round_start(board, index),
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

pub trait Initializable {
    fn init(board: &mut Board, index: usize);
}
pub trait RoundStarting {
    fn round_start(board: &mut Board, index: usize);
}
pub trait RoundEnding {
    fn round_end(board: &mut Board, index: usize);
}
pub trait SelfDestructing {
    fn round_end_self_destruct(board: &mut Board, index: usize);
}
pub trait AbilityUsing {
    fn use_ability(board: &mut Board, index: usize, on: usize);
}
pub trait ReactingToEXP {
    fn on_receive_exp(amount: u32, gold: &mut u32);
}
pub trait ReactingToTempStats {
    fn on_receive_temp_stats(board: &mut Board, index: usize);
}
pub trait ReactingToLevelup {
    fn on_levelup(tower: &mut Tower);
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum TowerRarity {
    Common,
    Uncommon,
    Rare,
    Malicious
}