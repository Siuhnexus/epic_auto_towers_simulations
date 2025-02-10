use std::collections::HashSet;

use crate::tower::Tower;

#[derive(Clone)]
pub struct Board {
    pub towers: [Option<Tower>; 15],
    pub relics: HashSet<u32>
}

impl Board {
    pub fn new() -> Board {
        Board { towers: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], relics: HashSet::new() }
    }
}