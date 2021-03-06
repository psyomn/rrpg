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
fn test_default_viewport() {
    let c = viewport::ViewportBuilder::new().finalize();

    assert_eq!((c.width(), c.height()), (0,0));
}

#[test]
fn test_map_shift_default() {
    let m = map::Map::new();
    assert_eq!(m.shift_x(), 0);
    assert_eq!(m.shift_y(), 0);
}


#[test]
fn test_entity_receive_damage() {
    let mut e = entity::Entity::new("default".to_string());
    let prev_hp = e.get_hitpoints();

    e.receive_damage(1);
    assert_eq!(e.get_hitpoints(), prev_hp - 1);

    e.receive_damage(10000000);
    assert_eq!(e.get_hitpoints(), 0);
}

#[test]
fn test_tile_add_entity() {
    let mut t = tile::Tile::new();
    let prev_count = t.entities().iter().count();

    let e = entity::Entity::new("jonny".to_string());
    t.add_entity(e);

    let next_count = t.entities().iter().count();
    assert_eq!(prev_count, 0);
    assert_eq!(next_count, 1);

    let ix = 100;

    for _ in 0..ix {
        let temp_e = entity::Entity::new("some ent".to_string());
        t.add_entity(temp_e);
    }

    let bigger_count = t.entities().iter().count();
    assert_eq!(bigger_count, ix + next_count);
}

