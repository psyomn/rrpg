extern crate rrpg;

use rrpg::entity::{Entity};
use rrpg::battle::{Battle};

fn main() {
    let heroes = vec!(
        Entity::new("Jonas".to_string()),
        Entity::new("Mary".to_string()),
        Entity::new("Dan".to_string()),
    );

    let enemies = vec!(
        Entity::new("goblin".to_string()),
        Entity::new("goblin".to_string()),
        Entity::new("goblin".to_string()),
        Entity::new("goblin".to_string()),
    );

    let mut battle = Battle::new();

    battle.set_enemies(enemies);
    battle.set_heroes(heroes);
    println!("{}", battle);
}
