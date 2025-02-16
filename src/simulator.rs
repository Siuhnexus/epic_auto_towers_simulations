use crate::{board::Board, logger::CSVLogger, stats::Stats, strategy::Strategy};

#[derive(Clone, PartialEq, Eq)]
pub enum LoggingStrategy {
    WholeRun,
    JustResults,
    RunWithMetaValue(u32)
}

impl LoggingStrategy {
    pub fn before_run(&self, logger: &mut CSVLogger, strategy: &impl Strategy) {
        if *self == LoggingStrategy::WholeRun {
            strategy.log_init(logger);
        }
    }
    pub fn log_round(&self, logger: &mut CSVLogger, strategy: &impl Strategy, board: &Board, round: u32) {
        match *self {
            LoggingStrategy::RunWithMetaValue(meta) => {
                logger.write(format!("{},", meta));
            },
            LoggingStrategy::JustResults => return,
            _ => {}
        }
        strategy.log(logger, board, round);
    }
    pub fn log_end(&self, logger: &mut CSVLogger, strategy: &impl Strategy, board: &Board, round: u32) {
        match *self {
            LoggingStrategy::RunWithMetaValue(meta) => {
                logger.write(format!("{},", meta));
            },
            _ => {}
        }
        strategy.log(logger, board, round);
    }
}

pub struct Simulator {
    pub logging: LoggingStrategy
}

impl Simulator {
    pub fn new(logging: LoggingStrategy) -> Simulator {
        Simulator { logging }
    }

    pub fn simulate(&self, logger: &mut CSVLogger, strategy: &impl Strategy) {
        let mut round = 1;
        let mut board = Board::new();

        strategy.init(&mut board, &mut round);

        let ending_round = strategy.last_round();

        self.logging.before_run(logger, strategy);
        self.logging.log_round(logger, strategy, &board, round);
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
            board.gold += 5;
            for i in 0..15 {
                match board.get_tower(i) {
                    None => {}, Some(v) => {
                        v.temp_stats = Stats::new(0.0, 0.0);
                        let kind = v.kind.clone();
                        kind.round_start(&mut board, i);
                    }
                }
            }
            if round > ending_round {
                self.logging.log_end(logger, strategy, &board, round);
            }
            else {
                self.logging.log_round(logger, strategy, &board, round);
            }
        }
    }
}