use std::fmt;
use entity::{Entity};

#[derive(Debug)]
pub struct Battle {
    enemies: Vec<Entity>,
    heroes: Vec<Entity>,
}

impl Battle {
    pub fn new() -> Battle {
        Battle {
            enemies: vec!(),
            heroes: vec!(),
        }
    }

    pub fn set_heroes(&mut self, heroes: Vec<Entity>) {
        self.heroes = heroes;
    }

    pub fn set_enemies(&mut self, enemies: Vec<Entity>) {
        self.enemies = enemies;
    }

    /// Step in battle
    pub fn step(self) {
    }
}

impl fmt::Display for Battle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_enemies = self.enemies.len();
        let num_heroes = self.heroes.len();
        write!(f, "Num Enemies: {}, Num Heroes: {}",
               num_enemies, num_heroes)
    }
}
