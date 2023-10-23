use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const BELT_TICK_SPEED: f32 = 5.0;
pub const BELT_CAPACITY: usize = 1;

fn main() {
    App::new()
        .register_type::<Belt>()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<BeltTicker>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                tick_belt_timer,
                tick_item_sink.after(tick_belt_timer),
                tick_belts.after(tick_item_sink),
                tick_belt_item_spawner.after(tick_belts),
            ),
        )
        .run();
}

#[derive(Component, Debug)]
// struct Name(String);
#[derive(Resource)]
struct BeltTicker {
    pub timer: Timer,
    pub tick_counter: u32,
}

impl BeltTicker {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(BELT_TICK_SPEED, TimerMode::Repeating),
            tick_counter: 0,
        }
    }
}

impl Default for BeltTicker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
struct Buildable {}

#[derive(Component)]
struct WorldItem(u8);

#[derive(Component, Debug, Reflect)]
struct Belt {
    // items on belt should be ordered like a queue
    items_on_belt: VecDeque<Entity>,
    next_belt: Entity,
}

#[derive(Bundle)]
struct WorldItemBundle{

}


#[derive(Component)]
struct ItemSpawner {
    item: WorldItem,
}



#[derive(Component)]
struct ItemSink {}

impl Belt {
    fn new_empty_belt() -> Belt {
        Belt {
            next_belt: Entity::PLACEHOLDER,
            items_on_belt: VecDeque::new(),
        }
    }
}

fn setup(mut commands: Commands) {
    let item_sink = commands
        .spawn((
            Belt::new_empty_belt(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            ItemSink {},
            Name::new("Item Sink"),
        ))
        .id();

    let belt1 = commands
        .spawn((
            Belt {
                items_on_belt: VecDeque::new(),
                next_belt: item_sink,
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Name::new("belt 1"),
        ))
        .id();

    let belt2 = commands
        .spawn((
            Belt {
                items_on_belt: VecDeque::new(),
                next_belt: belt1,
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Name::new("belt 2"),
        ))
        .id();

    let spawner = commands
        .spawn((
            Belt {
                items_on_belt: VecDeque::new(),
                next_belt: belt2,
            },
            ItemSpawner { item: WorldItem(1) },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Name::new("Item Spawner"),
        ))
        .id();
}

fn tick_belts(mut belt_query: Query<(Entity, &mut Belt, &Name)>, belt_ticker: Res<BeltTicker>) {
    if !belt_ticker.timer.finished() {
        return;
    }

    println!("------------------------------------------");
    println!("Belt Tick {}", belt_ticker.tick_counter);

    let mut belts: Vec<[Entity; 2]> = Vec::new();
    for (entity, belt, _) in belt_query.iter() {
        if belt.items_on_belt.len() > 0 && belt.next_belt != Entity::PLACEHOLDER {
            belts.push([entity, belt.next_belt]);
        }
    }

    for entities in belts {
        let [(_, mut belt_1, name_1), (_, mut belt_2, name_2)] = belt_query.many_mut(entities);

        if belt_2.items_on_belt.len() < BELT_CAPACITY {
            belt_2
                .items_on_belt
                .push_back(belt_1.items_on_belt.pop_front().unwrap());
        }
    }

    /*
    let mut belts: Vec<(Entity, &Belt, &Name)> = Vec::new();

    for (e, belt, name) in belt_query.iter() {
        let b = belt;
        belts.push((e, b, name));
    }

    for (entity, mut belt, name) in belts {
        if belt.items_on_belt.len() > 0 {
            if belt.next_belt != Entity::PLACEHOLDER {
                belts.
            }
        }
    }
    */

    //for mut belt in belt_query.iter_mut() {
    //    if belt.items_on_belt.len() > 0 {
    //        if (belt.next_belt != Entity::PLACEHOLDER) {
    //            println!("    - Checking for another belt!");

    // if let Ok((mut next_belt, transform)) = next_belt_query.get_mut(belt.next_belt) {

    //     // if next_belt.items_on_belt.len() < BELT_CAPACITY {
    //     //     // next_belt
    //     //     //     .items_on_belt
    //     //     //     .push_back(belt.items_on_belt.pop_front().unwrap());
    //     //
    //     // }
    // } else {
    // }
    // }
    //    }
    //}
}

fn tick_belt_item_spawner(
    mut belt_query: Query<(&ItemSpawner, &mut Belt)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (spawner, mut belt) in belt_query.iter_mut() {
        if belt.items_on_belt.len() < BELT_CAPACITY {
            let item = commands.spawn(WorldItem(0)).id();
            belt.items_on_belt.push_back(item);
        }
    }
}

fn tick_item_sink(mut belt_query: Query<(&ItemSink, &mut Belt)>, mut commands: Commands) {
    for (_, mut belt) in belt_query.iter_mut() {
        if belt.items_on_belt.len() > 0 {
            commands
                .entity(belt.items_on_belt.pop_front().unwrap())
                .despawn();
        }
    }
}

fn handle_input(input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        println!("Space was pressed");
    }
}

fn tick_belt_timer(mut belt_ticker: ResMut<BeltTicker>, time: Res<Time>) {
    belt_ticker.timer.tick(time.delta());
    if belt_ticker.timer.finished() {
        belt_ticker.tick_counter += 1;
    }
}
