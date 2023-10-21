use bevy::prelude::*;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tick_belts)
        .run();
}

#[derive(Component)]
struct Buildable {}

#[derive(Component)]
struct WorldItem(u8);

#[derive(Component, Debug)]
struct Belt {
    // items on belt should be ordered like a queue
    items_on_belt: [Entity; 1],
    next_belt: Entity,
}

impl Belt {
    fn new_empty_belt () -> Belt{
        Belt {
            items_on_belt: [Entity::PLACEHOLDER],
            next_belt: Entity::PLACEHOLDER,
        };
    }
}

fn setup(mut commands: Commands) {
    let belt1 = commands
        .spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((Belt::new_empty_belt(), Transform::from_xyz(0.0, 0.0, 0.0)));

    let belt2 = commands
        .spawn((
            Belt {
                items_on_belt: [Entity::PLACEHOLDER],
                next_belt: belt1,
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
}

fn tick_belts(mut belt_query: Query<&Belt>, mut next_belt_query: Query<(&Belt, &Transform)>) {
    for belt in belt_query.iter() {
        println!("Querying Belts!");
        if (belt.next_belt != Entity::PLACEHOLDER) {
            println!("Belt has an entity! {}", belt.next_belt.to_bits());
        }
        if let Ok((next_belt, transform)) = next_belt_query.get(belt.next_belt) {
            println!("Belt found!");
        } else {
            println!("NO NEXT BELT");
        }
        println!("Done querying belt!");
    }
}
