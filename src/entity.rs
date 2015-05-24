use std::fmt;

pub type entity_stat = i32;

#[derive(Debug)]
pub struct Entity {
    name:          String,
    level:        entity_stat,
    strength:     entity_stat,
    defense:      entity_stat,
    intelligence: entity_stat,
    mana:         entity_stat,
    hitpoints:    entity_stat,
    constitution: entity_stat,
}

impl Entity {
    pub fn new(n: String) -> Entity {
        Entity{
            name:          n.clone(),
            level:         1,
            strength:      3,
            defense:       1,
            intelligence:  5,
            mana:          5,
            hitpoints:    10,
            constitution: 10,
        }
    }

    pub fn attack(self) -> entity_stat {
        self.strength
    }

    pub fn defend(self) -> entity_stat {
        self.defense
    }

}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}: lvl {}]", self.name, self.level)
    }
}

