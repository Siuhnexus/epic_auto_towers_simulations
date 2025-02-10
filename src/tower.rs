use crate::stats::Stats;

#[derive(Clone)]
pub struct Tower {
    pub stats: Stats,
    pub exp: u32,
    pub level: u32,
    pub exp_required: u32,
    pub levelup_stats: Stats,
    pub chakra: u32,
}

impl Tower {
    pub fn new(attack: f64, life: f64, mut exp: u32, levelup_stats: Stats) -> Tower {
        let mut level = 1;
        let mut required = 10;
        while exp > required {
            level += 1;
            exp -= required;
            required += 5;
        }
        Tower { stats: Stats::new(attack, life), exp, level, exp_required: required, levelup_stats, chakra: 0 }
    }

    pub fn add_exp(&mut self, gained: u32) {
        self.exp += gained;
        while self.exp >= self.exp_required {
            self.level += 1;
            self.stats += self.levelup_stats;
            self.exp -= self.exp_required;
            self.exp_required += 5;
        }
    }
}