pub mod entity;
pub mod battle;
pub mod levelers;
pub mod map;
pub mod tile;
pub mod storyline_constants;
pub mod cli;
pub mod rrpg_rustbox;
pub mod viewport;

#[test]
fn test_map_tile_creation() {
    let default_map = map::Map::new();
    assert_eq!(default_map.x(), 10);
    assert_eq!(default_map.y(), 10);
}

#[test]
fn test_map_tile_creation_default_100_tiles() {
    let default_map = map::Map::new();
    assert_eq!(default_map.tiles().iter().count(), 100);
}

#[test]
fn test_new_map_contains_no_entities() {
    let default_map = map::Map::new();
    let count_entities: usize = default_map
        .tiles()
        .iter()
        .map(|ref t| t.entities().iter().count())
        .fold(0, |acc,item| acc + item);

    assert_eq!(count_entities, 0);
}

#[test]
fn test_map_default_viewport_width_80() {
    let def_map = map::Map::new();
    assert_eq!(def_map.max_w(), 80);
}

#[test]
fn test_map_default_viewport_height_40() {
    let def_map = map::Map::new();
    assert_eq!(def_map.max_h(), 40);
}
