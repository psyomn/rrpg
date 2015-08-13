pub mod fighter_leveler;


#[test]
fn levelup_fighter_test_name() {
    use entity::Entity;
    let mut e = Entity::new("potatoman".to_string());

    /* Before */
    assert_eq!("potatoman", e.get_name());
    assert_eq!(e.get_strength(), 3);
    assert_eq!(e.get_hitpoints(), 10);
    assert_eq!(e.get_constitution(), 10);

    /* After leveler */
    fighter_leveler::levelup_fighter(&mut e);
    assert_eq!("potatoman", e.get_name());
    assert_eq!(e.get_strength(), 5);
    assert_eq!(e.get_hitpoints(), 10);
    assert_eq!(e.get_constitution(), 12);
}
