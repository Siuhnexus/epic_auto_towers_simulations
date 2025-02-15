use crate::{board::Board, csv_line, logger::CSVLogger, strategy::Strategy, tower::Tower, tower_type::TowerType};

pub struct PrincessTest {
    pub princesses: usize
}

impl Strategy for PrincessTest {
    fn init(&self, board: &mut crate::board::Board, round: &mut u32) {
        *round = 11;
        board.set_tower(5, Tower::new(TowerType::Obsidian));
        for i in 0..self.princesses {
            board.set_tower(6 + i, Tower::new(TowerType::Princess));
        }
    }

    fn round_actions(&self, _board: &mut Board, _round: u32) {
        
    }

    fn log_init(&self, logger: &mut CSVLogger) {
        csv_line!(logger, "Round", "Level", "Attack", "Life", "EXP", "Levelup_EXP");
    }

    fn log(&self, logger: &mut CSVLogger, board: &crate::board::Board, round: u32) {
        let tower = board.towers[5].as_ref().unwrap();
        csv_line!(logger, round, tower.level, tower.stats.attack, tower.stats.life, tower.exp, tower.exp_required)
    }

    fn last_round(&self) -> u32 {
        106
    }
}