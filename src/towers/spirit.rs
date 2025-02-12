use crate::{board::Board, buffs::Buffs};

#[derive(Clone, Debug)]
pub struct Spirit {}

impl Spirit {
    pub fn init(board: &mut Board, index: usize) {
        board.add_exp(index, 6);
        let tower = board.get_tower(index).unwrap();
        tower.ability_uses = 1;
    }

    pub fn round_start(board: &mut Board, index: usize) {
        board.add_exp(index, 2);
        let tower = board.get_tower(index).unwrap();
        tower.ability_uses = tower.level;
    }

    pub fn use_ability(board: &mut Board, index: usize, on: usize) {
        let tower = board.get_tower(index).unwrap();
        if tower.ability_uses <= 0 { return; }
        tower.ability_uses -= 1;

        let target = board.get_tower(on).unwrap();
        *target.buffs.entry(Buffs::Chakra).or_insert(0) += 1;
    }
}