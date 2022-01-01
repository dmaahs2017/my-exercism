// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None
        }

        Some(Player {
            health: 100,
            mana: if self.level >= 10 {Some(100)} else {None},
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = &mut self.mana {
            if *mana >= mana_cost {
                *mana -= mana_cost;
                return mana_cost * 2;
            }
        } else {
            if let Some(new_hp) = self.health.checked_sub(mana_cost) {
                self.health = new_hp;
            } else {
                self.health = 0;
            }
        }

        0
    }
}
