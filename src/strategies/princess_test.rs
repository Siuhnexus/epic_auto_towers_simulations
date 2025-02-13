use crate::{board::Board, strategy::Strategy, tower::Tower, tower_type::TowerType};

pub struct PrincessTest {}

impl Strategy for PrincessTest {
    fn init(&self, board: &mut crate::board::Board, round: &mut u32) {
        *round = 11;
        board.set_tower(5, Tower::new(TowerType::Obsidian));
        board.set_tower(6, Tower::new(TowerType::Princess));
        board.set_tower(7, Tower::new(TowerType::Princess));
        board.set_tower(8, Tower::new(TowerType::Princess));
    }

    fn round_actions(&self, _board: &mut Board, _round: u32) {
        
    }

    fn log_init(&self) -> String {
        format!("Round,Level,Attack,Life,EXP,Levelup_EXP")
    }

    fn log(&self, board: &crate::board::Board, round: u32) -> String {
        let tower = board.towers[5].as_ref().unwrap();
        format!("{},{},{},{},{},{}", round, tower.level, tower.stats.attack, tower.stats.life, tower.exp, tower.exp_required)
    }

    fn last_round(&self) -> u32 {
        106
    }
}