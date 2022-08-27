use bevy::{prelude::*, window::close_on_esc};
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
        .insert_resource(Spawner(Timer::from_seconds(1., true)))
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
        .add_system(spawn_shapes)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_ground(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0., -250., Z)))
        .insert(Collider::cuboid(250., 20.));
}

struct Spawner(Timer);

#[derive(Component)]
enum Item {
    Rice,
    SeaWeed,
    Avocado,
    Fish,
}

fn spawn_shapes(mut commands: Commands, mut spawner: ResMut<Spawner>, time: Res<Time>) {
    if spawner.0.tick(time.delta()).just_finished() {
        let (item, collider) = generate_item(&time);

        commands
            .spawn()
            .insert(item)
            .insert_bundle(TransformBundle::from(Transform::from_xyz(0., 100., 0.)))
            .insert(RigidBody::Dynamic)
            .insert(collider)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

fn generate_item(time: &Time) -> (Item, Collider) {
    match (time.seconds_since_startup() * 1000.) as i32 % 4 {
        0 => (Item::SeaWeed, Collider::cuboid(30., 30.)),
        1 => (Item::Rice, Collider::ball(30.)),
        2 => (Item::Fish, Collider::capsule_x(15., 30.)),
        _ => (Item::Avocado, Collider::round_cuboid(10., 10., 0.3)),
    }
}

#[derive(Debug, Default)]
struct GrabbedItem(Option<Entity>);

fn drag_and_drop_item(
    mouse: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut items: Query<(Entity, &mut Transform, &Collider), With<Item>>,
    mut grabbed_item: ResMut<GrabbedItem>,
) {
    if mouse.just_released(MouseButton::Left) {
        grabbed_item.0 = None;
        return;
    }
    if mouse.just_pressed(MouseButton::Left) {
        grabbed_item.0 = items
            .iter()
            .find(|(_, transform, collider)| {
                collider.contains_local_point(mouse_position.0 - transform.translation.truncate())
            })
            .map(|(entity, ..)| entity);
    }

    if let Some(item) = grabbed_item.0 {
        let mut transform = items
            .get_component_mut::<Transform>(item)
            .expect("items contains transform");

        transform.translation = Vec3::from((mouse_position.0, Z));
    }
}