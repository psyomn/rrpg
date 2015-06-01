use std::fmt;
use entity::{Entity, EntityStat};
use std::io;
use std::io::prelude::*;

extern crate term;

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
        let attack_str = "attack".to_string();
        let mut choice = "default".to_string();
        while !self.done {
            self.show_heroes();
            self.show_enemies();

            print!("> ");
            io::stdout().flush().ok().expect("could not flush");

            /* TODO funky way of doing getline in rust. Maybe change when API is better for this... */
            for line in stdin.lock().lines() {
                choice = line.unwrap();
                break;
            }

            if choice == "attack".to_string() || choice == "a".to_string() {
                println!("The heroes attack!");
                self.heroes_attack();
            }
            else if choice == "defend".to_string() || choice == "d".to_string() {
                println!("The heroes defend!");
                self.heroes_defend();
            }
            else if choice == "exit".to_string() || choice == "x".to_string() {
                self.done = true;
            }
            else {
                /* Print help */
                println!("These are your options:");
                println!(" - attack,a");
                println!(" - defend,d");
                println!(" - exit,x");
            };

            self.remove_dead_enemies();
        }
    }

    fn heroes_attack(&mut self) {
        let mut attacks_cycle = self.heroes.iter().map(|h| h.attack()).collect::<Vec<EntityStat>>();
        let mut en_iter = self.enemies.iter_mut();

        for e in en_iter {
            if attacks_cycle.len() == 0 { break; }
            let x = attacks_cycle.pop();
            match x {
                Some(x) => {
                    let v = x;
                    e.receive_damage(v);
                },
                _ => e.receive_damage(0),
            }
        }
    }

    fn heroes_defend(&self) {
        let h_defend = self.enemies.iter();
    }

    fn get_dead_enemy_indices(&self) -> Vec<usize> {
        let mut en_it =
            self.enemies.iter().enumerate();
        let mut indices = vec!();

        for (ix, enemy) in en_it {
            if enemy.get_hitpoints() == 0 { indices.push(ix) }
        }

        indices.sort_by(|a,b| b.cmp(a));
        indices
    }

    fn remove_dead_enemies(&mut self) {
        let indices = self.get_dead_enemy_indices();
        for i in indices {
            self.enemies.remove(i);
        }
    }

    pub fn show_heroes(&self) {
        let mut t = term::stdout().unwrap();
        for e in self.heroes.iter() {
            t.fg(term::color::GREEN).unwrap();
            print!("* ");
            t.reset().unwrap();
            println!("{}", e);
        }
    }

    pub fn show_enemies(&self) {
        let mut t = term::stdout().unwrap();
        for e in self.enemies.iter() {
            t.fg(term::color::RED).unwrap();
            print!("* ");
            t.reset().unwrap();
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
