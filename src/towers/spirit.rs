use crate::{board::Board, buffs::Buffs, tower_type::{AbilityUsing, Initializable, RoundStarting}};

pub struct Spirit {}

impl Initializable for Spirit {
    fn init(board: &mut Board, index: usize) {
        board.add_exp(index, 6);
        let tower = board.get_tower(index).unwrap();
        tower.ability_uses = 1;
    }
}
impl RoundStarting for Spirit {
    fn round_start(board: &mut Board, index: usize) {
        board.add_exp(index, 2);
        let tower = board.get_tower(index).unwrap();
        tower.ability_uses = tower.level;
    }
}
impl AbilityUsing for Spirit {
    fn use_ability(board: &mut Board, index: usize, on: usize) {
        let tower = board.get_tower(index).unwrap();
        if tower.ability_uses <= 0 { return; }
        tower.ability_uses -= 1;

        let target = board.get_tower(on).unwrap();
        *target.buffs.entry(Buffs::Chakra).or_insert(0) += 1;
    }
}