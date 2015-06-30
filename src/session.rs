use entity::{Entity};

struct Session {
    party: Vec<Entity>,
}

impl Session {
    fn new() -> Session {
        Session { party: vec!() }
    }

    fn add_hero(&mut self, e: &Entity) {
        self.party.push(e.clone());
    }
}
