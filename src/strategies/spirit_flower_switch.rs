use crate::{board::Board, buffs::Buffs, csv_line, logger::CSVLogger, relics::Relic, stats::Stats, strategy::Strategy, tower::Tower, tower_type::TowerType};

pub struct SpiritFlowerSwitch {
    pub round: u32
}
impl Strategy for SpiritFlowerSwitch {
    fn init(&self, board: &mut crate::board::Board, round: &mut u32) {
        *round = 22;
        board.add_relic(Relic::Hourglass);
        board.set_tower(0, Tower::new(TowerType::Spirit));
        let mut obsidian = Tower::new(TowerType::Obsidian);
        obsidian.add_exp(25);
        obsidian.stats = Stats::new(100.0, 100.0);
        board.set_tower(1, obsidian);
        let mut flower = Tower::new(TowerType::Flower);
        flower.add_exp(45);
        flower.stats = Stats::new(100.0, 100.0);
        board.set_tower(2, flower);
    }

    fn round_actions(&self, board: &mut Board, round: u32) {
        board.set_tower(3, Tower::new(TowerType::Honey));
        if board.gold > 0 { board.gold -= 1 };
        let obsidians = (board.gold as f64 * 0.66 * 0.2).floor() as u32;
        board.add_exp(1, obsidians * 5);
        board.gold = 0;

        if round >= self.round {
            let tower = board.get_tower(0).unwrap();
            let uses = tower.ability_uses;
            let kind = tower.kind.clone();
            for _ in 0..uses {
                kind.use_ability(board, 0, 2);
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
        })
    }

    fn last_round(&self) -> u32 {
        68
    }
}