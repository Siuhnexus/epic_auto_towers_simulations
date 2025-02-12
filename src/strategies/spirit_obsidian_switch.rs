use crate::{board::Board, buffs::Buffs, relics::Relic, stats::Stats, strategy::Strategy, tower::Tower, tower_type::TowerType};

pub struct SpiritObsidianSwitch {
    pub round: u32
}
impl Strategy for SpiritObsidianSwitch {
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

    fn log_init(&self) -> String {
        format!("Round,Level,Attack,Life,EXP,Levelup_EXP,Chakra")
    }

    fn log(&self, board: &crate::board::Board, round: u32) -> String {
        let tower = board.towers[1].as_ref().unwrap();
        format!("{},{},{},{},{},{},{}", round, tower.level, tower.stats.attack, tower.stats.life, tower.exp, tower.exp_required, match tower.buffs.get(&Buffs::Chakra) {
            None => 0, Some(v) => *v
        })
    }

    fn last_round(&self) -> u32 {
        68
    }
}