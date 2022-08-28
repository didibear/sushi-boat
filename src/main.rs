#![deny(clippy::all, clippy::nursery, clippy::unwrap_used)]

use bevy::{prelude::*, utils::HashSet, window::close_on_esc};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use mouse_tracking::{MousePosition, MouseTrackingPlugin};
use rand::Rng;

mod mouse_tracking;
const Z: f32 = 0.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Stacker".to_string(),
            width: 800.,
            height: 800.,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::default())
        .insert_resource(Spawner(Timer::from_seconds(3., true)))
        .insert_resource(GrabbedItem::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(MouseTrackingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::GamePlay)
                .with_collection::<ItemAssets>(),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GamePlay)
                .with_system(setup_camera)
                .with_system(setup_ground),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GamePlay)
                .with_system(spawn_incoming_items)
                .with_system(drag_and_drop_item)
                .with_system(combine_items),
        )
        .add_system(close_on_esc)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    GamePlay,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_ground(mut commands: Commands) {
    commands
        .spawn()
        .insert(Name::new("Ground"))
        .insert(Collider::cuboid(250., 20.))
        .insert(Friction::new(1.2))
        .insert_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., -250., Z),
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(250., 20.) * 2.),
                ..default()
            },
            ..default()
        });
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Item {
    // Basic
    Rice,
    SeaWeed,
    Avocado,
    Fish,
    // Level 1
    Onigiri,
    Maki,
    Sushi,
    // Level 2
    MakiSushiTray,
}

impl Item {
    const BASIC: [Self; 4] = [Self::Rice, Self::SeaWeed, Self::Avocado, Self::Fish];

    fn can_combine(item1: Self, item2: Self) -> Option<Self> {
        use Item::*;

        match sorted([item1, item2]) {
            [Rice, SeaWeed] => Some(Onigiri),
            [Rice, Fish] => Some(Sushi),
            [Avocado, Onigiri] => Some(Maki),
            [Maki, Sushi] => Some(MakiSushiTray),
            _ => None,
        }
    }
}

impl From<Item> for Collider {
    fn from(item: Item) -> Self {
        match item {
            Item::Rice => Self::ball(30.),
            Item::SeaWeed => Self::cuboid(30., 30.),
            Item::Avocado => Self::capsule_x(10., 20.),
            Item::Fish => Self::capsule_x(15., 30.),
            Item::Onigiri => Self::ball(40.),
            Item::Maki => Self::cuboid(30., 40.),
            Item::Sushi => Self::capsule_x(30., 30.),
            Item::MakiSushiTray => Self::cuboid(50., 50.),
        }
    }
}

#[derive(AssetCollection)]
struct ItemAssets {
    #[asset(path = "sprites/rice.png")]
    rice: Handle<Image>,
    #[asset(path = "sprites/sea-weed.png")]
    sea_weed: Handle<Image>,
    #[asset(path = "sprites/fish.png")]
    fish: Handle<Image>,
    #[asset(path = "sprites/avocado.png")]
    avocado: Handle<Image>,
    #[asset(path = "sprites/onigiri.png")]
    onigiri: Handle<Image>,
    #[asset(path = "sprites/maki.png")]
    maki: Handle<Image>,
    #[asset(path = "sprites/sushi.png")]
    sushi: Handle<Image>,
    #[asset(path = "sprites/maki-sushi-tray.png")]
    maki_sushi_tray: Handle<Image>,
}

impl ItemAssets {
    fn get(&self, item: Item) -> (Handle<Image>, Sprite) {
        let sprite = Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        };
        match item {
            Item::Rice => (self.rice.clone(), sprite),
            Item::SeaWeed => (self.sea_weed.clone(), sprite),
            Item::Avocado => (self.avocado.clone(), sprite),
            Item::Fish => (self.fish.clone(), sprite),
            Item::Onigiri => (self.onigiri.clone(), sprite),
            Item::Maki => (self.maki.clone(), sprite),
            Item::Sushi => (self.sushi.clone(), sprite),
            Item::MakiSushiTray => (self.maki_sushi_tray.clone(), sprite),
        }
    }
}

struct Spawner(Timer);

fn spawn_incoming_items(
    mut commands: Commands,
    mut spawner: ResMut<Spawner>,
    time: Res<Time>,
    item_assets: Res<ItemAssets>,
) {
    if spawner.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let item = Item::BASIC[rng.gen_range(0..Item::BASIC.len())];

        let side = [-1., 1.][rng.gen_range(0..2)]; // left or right

        let translation = Vec2::new(450. * side, rng.gen_range(0.0..300.0));

        let velocity = Velocity {
            linvel: Vec2::new(-side, 1.) * rng.gen_range(150.0..200.0),
            angvel: rng.gen_range(-10.0..10.0),
        };

        spawn_item(&mut commands, item, translation, velocity, &item_assets);
    }
}

fn spawn_item(
    commands: &mut Commands,
    item: Item,
    translation: Vec2,
    velocity: Velocity,
    item_assets: &Res<ItemAssets>,
) {
    let transform = Transform::from_translation(Vec3::from((translation, Z)));
    let (texture, sprite) = item_assets.get(item);

    commands
        .spawn()
        .insert(Name::new("Item"))
        .insert(item)
        .insert_bundle(SpriteBundle {
            texture,
            transform,
            sprite,
            ..default()
        })
        // .insert_bundle(TransformBundle::from(transform))
        .insert(RigidBody::Dynamic)
        .insert(Collider::from(item))
        .insert(velocity)
        .insert(Ccd::enabled())
        .insert(GravityScale(3.))
        .insert(ActiveEvents::COLLISION_EVENTS);
}

#[derive(Debug, Default)]
struct GrabbedItem(Option<Entity>);

fn drag_and_drop_item(
    mouse: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut items: Query<(Entity, &Collider, &Transform, &mut Velocity), With<Item>>,
    mut grabbed_item: ResMut<GrabbedItem>,
) {
    if mouse.just_released(MouseButton::Left) {
        if let Some(item) = grabbed_item.0.take() {
            let (.., mut velocity) = items.get_mut(item).expect("item has body");
            velocity.linvel = velocity.linvel.clamp_length_max(500.); // Cap speed when the player throw the item
        }
        return;
    }
    if mouse.just_pressed(MouseButton::Left) {
        grabbed_item.0 = items
            .iter()
            .find(|(_, collider, transform, _)| {
                collider.contains_local_point(mouse_position.0 - transform.translation.truncate())
            })
            .map(|(entity, ..)| entity);
    }

    if let Some(item) = grabbed_item.0 {
        // Move the grabbed item to the mouse cursor using the velocity
        let (.., transform, mut velocity) = items.get_mut(item).expect("item has body");
        velocity.linvel = (mouse_position.0 - transform.translation.truncate()) * 10.;
        velocity.angvel *= 0.9; // Smoothly decelerate the rotations
    }
}

fn combine_items(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    items: Query<(Entity, &Item, &Transform)>,
    mut grabbed_item: ResMut<GrabbedItem>,
    item_assets: Res<ItemAssets>,
) {
    let collided_items = collision_events
        .iter()
        .filter_map(|event| match event {
            CollisionEvent::Started(e1, e2, ..) => Some(sorted([e1, e2])),
            _ => None,
        })
        .collect::<HashSet<_>>() // Remove double-counted collisions
        .into_iter()
        .filter_map(|[e1, e2]| items.get_many([*e1, *e2]).ok());

    for [(entity1, item1, transform1), (entity2, item2, transform2)] in collided_items {
        if let Some(combined_item) = Item::can_combine(*item1, *item2) {
            grabbed_item.0 = None;
            commands.entity(entity1).despawn_recursive();
            commands.entity(entity2).despawn_recursive();

            let in_between_translation = (transform1.translation + transform2.translation) / 2.;

            spawn_item(
                &mut commands,
                combined_item,
                in_between_translation.truncate(),
                Velocity::zero(),
                &item_assets,
            );
        }
    }
}

fn sorted<const N: usize, T: Ord>(mut array: [T; N]) -> [T; N] {
    array.sort();
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_can_combine() {
        use Item::*;

        let result = Item::can_combine(Rice, Fish);

        assert_eq!(result, Some(Sushi));
        assert_eq!(result, Item::can_combine(Fish, Rice));
        //                                   ^ swapped items
    }
}
