use epic_auto_towers_simulations::{board::Board, buffs::Buffs, simulator::Simulator, strategy::Strategy, tower::Tower, towers::spirit::Spirit};

struct SpiritMaxLevel {}
impl Strategy for SpiritMaxLevel {
    fn init(&self, board: &mut Board, _gold: &mut u32, round: &mut u32) {
        *round = 22;
        board.set_tower(0, Tower::new(Box::new(Spirit {})));
    }

    fn round_actions(&self, board: &mut Board, _gold: &mut u32) {
        let spirit = board.get_tower(0).unwrap();
        let mut uses = spirit.ability_uses;
        let kind = spirit.kind.clone();
        while uses > 0 {
            kind.use_ability(board, 0, 0);
            uses -= 1;
        }
    }

    fn last_round(&self) -> u32 {
        106
    }
    
    fn log_init(&self) -> String {
        format!("Round,Level,Attack,Life,EXP,Levelup_EXP,Chakra")
    }
    fn log(&self, board: &Board, _gold: u32, round: u32) -> String {
        let tower = board.towers[0].as_ref().unwrap();
        format!("{},{},{},{},{},{},{}", round, tower.level, tower.stats.attack, tower.stats.life, tower.exp, tower.exp_required, match tower.buffs.get(&Buffs::Chakra) {
            None => 0, Some(v) => *v
        })
    }
}

fn main() {
    let sim = Simulator::new();
    sim.simulate(SpiritMaxLevel {});
}
