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
fn test_map_default_viewable_w() {
    let def_map = map::Map::new();
    assert!(def_map.x() != 0);
    assert_eq!(
        def_map.max_viewport_w(),
        cli::cli_constants::SCREEN_WIDTH);
}

#[test]
fn test_map_default_viewable_h() {
    let def_map = map::Map::new();
    assert!(def_map.y() != 0);
    assert_eq!(
        def_map.max_viewport_h(),
        cli::cli_constants::SCREEN_HEIGHT);
}

#[test]
fn test_viewport_set_x_y_test() {
    let set_x = 10;
    let set_y = 10;
    let c = viewport::ViewportBuilder::new()
        .x(set_x)
        .y(set_y)
        .finalize();

    assert_eq!(c.width(), set_x);
    assert_eq!(c.height(), set_y);
}

#[test]
fn test_map_shift_default() {
    let m = map::Map::new();
    assert_eq!(m.shift_x(), 0);
    assert_eq!(m.shift_y(), 0);
}

