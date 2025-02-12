use crate::{board::Board, stats::Stats, strategy::Strategy};

pub struct Simulator {}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator {}
    }

    pub fn simulate(&self, strategy: impl Strategy) {
        let mut round = 1;
        let mut board = Board::new();

        strategy.init(&mut board, &mut round);

        let ending_round = strategy.last_round();

        //println!("{}", strategy.log_init());
        //println!("{}", strategy.log(&board, round));
        while round <= ending_round {
            // Action phase
            strategy.round_actions(&mut board, round);
            // Round end
            for i in 0..15 {
                match board.get_tower(i) {
                    None => {}, Some(v) => {
                        let kind = v.kind.clone();
                        kind.round_end(&mut board, i);
                    }
                }
            }
            for i in 0..15 {
                match board.get_tower(i) {
                    None => {}, Some(v) => {
                        let buffs = v.buffs.clone();
                        for (kind, amount) in buffs {
                            kind.round_end(&mut board, i, amount);
                        }
                    }
                }
            }
            for i in 0..15 {
                match board.get_tower(i) {
                    None => {}, Some(v) => {
                        let kind = v.kind.clone();
                        kind.round_end_self_destruct(&mut board, i);
                    }
                }
            }
            round += 1;
            // Round start
            for i in 0..15 {
                match board.get_tower(i) {
                    None => {}, Some(v) => {
                        v.temp_stats = Stats::new(0.0, 0.0);
                        let kind = v.kind.clone();
                        kind.round_start(&mut board, i);
                    }
                }
            }
            //println!("{}", strategy.log(&board, round));
        }
        println!("{}", strategy.log(&board, round));
    }
}