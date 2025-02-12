use crate::stats::Stats;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Relic {
    Hourglass
}

impl Relic {
    pub fn on_temp_stats(&self, stats: Stats) -> Stats {
        match self {
            Relic::Hourglass => {
                stats * 1.5
            }
        }
    }
}