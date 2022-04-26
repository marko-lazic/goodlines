use bevy::utils::default;
use bevy::window::WindowDescriptor;

const APP_WIDTH: f32 = 546.0;
const APP_HEIGHT: f32 = 364.0;

pub fn window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        width: APP_WIDTH,
        height: APP_HEIGHT,
        title: "Good Lines".to_string(),
        ..default()
    }
}
