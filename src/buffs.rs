use crate::{stats::Stats, tower::Tower};

#[derive(Hash, Clone, PartialEq, Eq)]
pub enum Buffs {
    Chakra
}

impl Buffs {
    pub fn round_end(&self, parent: &mut Tower, amount: u32) {
        match self {
            Buffs::Chakra => {
                parent.stats += Stats::new(2.0, 3.0) * amount;
            }
            _ => {}
        }
    }
}