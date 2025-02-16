use crate::{board::Board, buffs::Buffs, csv_line, logger::CSVLogger, strategy::Strategy, tower::Tower, tower_type::TowerType};

pub struct SpiritLadderSwitch {
    pub round: u32
}
impl Strategy for SpiritLadderSwitch {
    fn init(&self, board: &mut crate::board::Board, round: &mut u32) {
        *round = 47;
        let mut spirit = Tower::new(TowerType::Spirit);
        spirit.add_exp(1357);
        spirit.buffs.insert(Buffs::Chakra, 169);
        board.set_tower(0, spirit);
        let mut ladder = Tower::new(TowerType::Ladder);
        ladder.add_exp(10);
        board.set_tower(1, ladder);
        let mut debt = Tower::new(TowerType::Debt);
        debt.add_exp(10);
        board.set_tower(5, debt);
        board.get_tower(5).unwrap().x_value = 11;
        debt = Tower::new(TowerType::Debt);
        debt.add_exp(10);
        board.set_tower(6, debt);
        board.get_tower(6).unwrap().x_value = 20;
        debt = Tower::new(TowerType::Debt);
        debt.add_exp(10);
        board.set_tower(7, debt);
        board.get_tower(7).unwrap().x_value = 17;
        debt = Tower::new(TowerType::Debt);
        debt.add_exp(10);
        board.set_tower(11, debt);
        board.get_tower(11).unwrap().x_value = 9;
        debt = Tower::new(TowerType::Debt);
        debt.add_exp(10);
        board.set_tower(13, debt);
        board.get_tower(13).unwrap().x_value = 31;
    }

    fn round_actions(&self, board: &mut Board, round: u32) {
        if round >= self.round {
            let tower = board.get_tower(0).unwrap();
            let mut uses = tower.ability_uses;
            let kind = tower.kind.clone();
            uses = uses.min(board.gold / 3);
            for _ in 0..uses {
                kind.use_ability(board, 0, 1);
                board.gold -= 3;
            }
        }
        else {
            let tower = board.get_tower(0).unwrap();
            let mut uses = tower.ability_uses;
            let kind = tower.kind.clone();
            uses = uses.min(board.gold / 3);
            for _ in 0..uses {
                kind.use_ability(board, 0, 0);
                board.gold -= 3;
            }
        }
    }

    fn log_init(&self, logger: &mut CSVLogger) {
        csv_line!(logger, "Gold", "Round", "Level", "XSum", "Chakra");
    }

    fn log(&self, logger: &mut CSVLogger, board: &crate::board::Board, round: u32) {
        let tower = board.towers[1].as_ref().unwrap();
        let level = tower.level;
        let chakra = match tower.buffs.get(&Buffs::Chakra) {
            None => 0, Some(v) => *v
        };
        let mut x_sum = 0;
        for i in [5, 6, 7, 11, 13] {
            x_sum += board.get_tower_ref(i).unwrap().x_value;
        }
        csv_line!(logger, board.gold, round, level, x_sum, chakra);
    }

    fn last_round(&self) -> u32 {
        104
    }
}