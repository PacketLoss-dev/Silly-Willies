use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component)]
struct WorldItem {
    item_id: u8,
}

#[derive(Component)]
struct Buildable {}

#[derive(Component)]
struct Belt {
    // items on belt should be ordered like a queue
    items_on_belt:[Entity;2],
    
}


