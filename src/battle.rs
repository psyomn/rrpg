use std::fmt;
use entity::{Entity};
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Battle {
    enemies: Vec<Entity>,
    heroes: Vec<Entity>,
    done: bool,
}

impl Battle {
    pub fn new() -> Battle {
        Battle {
            enemies: vec!(),
            heroes: vec!(),
            done: false,
        }
    }

    pub fn set_heroes(&mut self, heroes: Vec<Entity>) {
        self.heroes = heroes;
    }

    pub fn set_enemies(&mut self, enemies: Vec<Entity>) {
        self.enemies = enemies;
    }

    /// Step in battle
    pub fn fight(&mut self) {
        let stdin = io::stdin();
        while !self.done {
            self.show_heroes();
            self.show_enemies();

            for line in stdin.lock().lines() {
                let l = line.unwrap();
            }
        }
    }

    pub fn show_heroes(&self) {
        for e in self.heroes.iter() {
            println!("{}", e);
        }
    }

    pub fn show_enemies(&self) {
        for e in self.enemies.iter() {
            println!("{}", e);
        }
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
