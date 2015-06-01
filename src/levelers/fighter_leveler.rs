use entity::{Entity};

pub fn levelup_fighter(e: &mut Entity) {
    let curr_str   = e.get_strength();
    let curr_const = e.get_constitution();
    let next_str   = curr_str + 2;
    let next_const = curr_const + 2;
    e.set_strength(next_str);
    e.set_constitution(next_const);
}
