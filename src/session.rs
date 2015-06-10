use entity::{Entity};

struct Session {
    party: Vec<Entity>,
}

impl Session {
    fn new() -> Session {
        Session { party: vec!() }
    }
}
