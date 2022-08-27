use bevy::prelude::*;

pub struct MouseTrackingPlugin;

impl Plugin for MouseTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition::default())
            .add_system(track_mouse_position);
    }
}

#[derive(Debug, Default)]
pub struct MousePosition(pub Vec2);

fn track_mouse_position(
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    if let Some(cursor) = cursor_moved_events.iter().last() {
        let window = windows.primary();
        mouse_position.0 = cursor.position - Vec2::new(window.width(), window.height()) / 2.
    }
}
