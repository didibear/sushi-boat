#![deny(clippy::all, clippy::nursery, clippy::unwrap_used)]

use bevy::{prelude::*, utils::HashSet, window::close_on_esc};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use mouse_tracking::{MousePosition, MouseTrackingPlugin};

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
        .insert_resource(Spawner(Timer::from_seconds(2., true)))
        .insert_resource(GrabbedItem::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(MouseTrackingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ground)
        .add_system(drag_and_drop_item)
        .add_system(close_on_esc)
        .add_system(spawn_items)
        .add_system(combine_items)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_ground(mut commands: Commands) {
    commands
        .spawn()
        .insert(Name::new("Ground"))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0., -250., Z)))
        .insert(Collider::cuboid(250., 20.))
        .insert(Friction::new(1.2));
}

struct Spawner(Timer);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Item {
    // Basic Items
    Rice,
    SeaWeed,
    Avocado,
    Fish,
    // Combined 1
    Onigiri,
    Maki,
    Sushi,
}

impl Item {
    fn random_basic(time: &Time) -> Self {
        const BASIC_ITEMS: [Item; 4] = [Item::Rice, Item::SeaWeed, Item::Avocado, Item::Fish];

        BASIC_ITEMS[(time.seconds_since_startup() * 1000.) as usize % BASIC_ITEMS.len()]
    }

    fn can_combine(item1: Self, item2: Self) -> Option<Self> {
        use Item::*;

        match sorted([item1, item2]) {
            [Rice, SeaWeed] => Some(Onigiri),
            [Rice, Fish] => Some(Sushi),
            [Avocado, Onigiri] => Some(Maki),
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
        }
    }
}

fn spawn_items(mut commands: Commands, mut spawner: ResMut<Spawner>, time: Res<Time>) {
    if spawner.0.tick(time.delta()).just_finished() {
        let item = Item::random_basic(&time);
        spawn_item(&mut commands, item, Vec2::new(0., 200.));
    }
}

fn spawn_item(commands: &mut Commands, item: Item, translation: Vec2) {
    let transform = Transform::from_translation(Vec3::from((translation, Z)));

    commands
        .spawn()
        .insert(Name::new("Item"))
        .insert(item)
        .insert_bundle(TransformBundle::from(transform))
        .insert(RigidBody::Dynamic)
        .insert(Collider::from(item))
        .insert(Velocity::zero())
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
