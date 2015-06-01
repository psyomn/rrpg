use std::fmt;

extern crate term;

pub type EntityStat = i32;

#[derive(Debug)]
pub struct Entity {
    name:         String,
    level:        EntityStat,
    strength:     EntityStat,
    defense:      EntityStat,
    intelligence: EntityStat,
    mana:         EntityStat,
    hitpoints:    EntityStat,
    constitution: EntityStat,
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

    pub fn get_hitpoints(&self) -> EntityStat {
        self.hitpoints
    }

    pub fn get_constitution(&self) -> EntityStat {
        self.constitution
    }

    pub fn set_constitution(&mut self, c: EntityStat) {
        self.constitution = c;
    }

    pub fn get_strength(&self) -> EntityStat {
        self.strength
    }

    pub fn set_strength(&mut self, s: EntityStat) {
        self.strength = s;
    }

    pub fn attack(&self) -> EntityStat {
        self.strength
    }

    pub fn defend(&self) -> EntityStat {
        self.defense
    }

    /// Safe way to decrement hitpoints
    pub fn receive_damage(&mut self, e: EntityStat) {
        if self.hitpoints > e {
            self.hitpoints -= e;
            return;
        }
        self.hitpoints = 0;
    }

}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut t = term::stdout().unwrap();
        write!(f, "[").unwrap();
        t.fg(term::color::BLUE).unwrap();
        write!(f, "{}", self.name).unwrap();
        t.reset().unwrap();
        write!(f, ":").unwrap();

        t.fg(term::color::MAGENTA).unwrap();
        write!(f, " lvl ").unwrap();
        t.reset().unwrap();

        t.fg(term::color::YELLOW).unwrap();
        write!(f, "{}", self.level).unwrap();
        t.reset().unwrap();

        t.fg(term::color::GREEN).unwrap();
        write!(f, " {}/{}", self.hitpoints, self.constitution);
        t.reset().unwrap();

        write!(f, "]")
    }
}

