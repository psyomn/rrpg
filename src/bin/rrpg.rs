extern crate rrpg;

use rrpg::entity::{Entity};
use rrpg::battle::{Battle};
use rrpg::levelers::fighter_leveler;

fn main() {
    let mut dan = Entity::new("Dan".to_string());

    fighter_leveler::levelup_fighter(&mut dan);

    let heroes = vec!(
        Entity::new("Jonas".to_string()),
        Entity::new("Mary".to_string()),
        dan,
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

    battle.fight();

    println!("{}", battle);
}
