use crate::{board::Board, buffs::Buffs, csv_line, logger::CSVLogger, strategy::Strategy, tower::Tower, tower_type::TowerType};

pub struct SpiritLadderSwitch {
    pub round: u32
}
impl Strategy for SpiritLadderSwitch {
    fn init(&self, board: &mut crate::board::Board, round: &mut u32) {
        *round = 22;
        board.set_tower(0, Tower::new(TowerType::Spirit));
        let ladder = Tower::new(TowerType::Ladder);
        board.set_tower(1, ladder);
    }

    fn round_actions(&self, board: &mut Board, round: u32) {
        if round >= self.round {
            let tower = board.get_tower(0).unwrap();
            let uses = tower.ability_uses;
            let kind = tower.kind.clone();
            for _ in 0..uses {
                kind.use_ability(board, 0, 1);
            }
        }
        else {
            let tower = board.get_tower(0).unwrap();
            let uses = tower.ability_uses;
            let kind = tower.kind.clone();
            for _ in 0..uses {
                kind.use_ability(board, 0, 0);
            }
        }
    }

    fn log_init(&self, logger: &mut CSVLogger) {
        csv_line!(logger, "Round", "Level", "Attack", "Life", "EXP", "Levelup_EXP", "Chakra");
    }

    fn log(&self, logger: &mut CSVLogger, board: &crate::board::Board, round: u32) {
        let tower = board.towers[1].as_ref().unwrap();
        csv_line!(logger, round, tower.level, tower.stats.attack, tower.stats.life, tower.exp, tower.exp_required, match tower.buffs.get(&Buffs::Chakra) {
            None => 0, Some(v) => *v
        });
    }

    fn last_round(&self) -> u32 {
        91
    }
}