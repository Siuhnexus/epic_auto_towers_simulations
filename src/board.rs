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

    pub fn get_tower(&mut self, index: usize) -> Option<&mut Tower> {
        match self.towers.get_mut(index) {
            None => None, Some(v) => v.as_mut()
        }
    }

    pub fn set_tower(&mut self, index: usize, tower: Tower) {
        let kind= tower.kind.clone();
        self.towers[index] = Some(tower);
        kind.init(self, index);
    }
}