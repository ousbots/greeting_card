use bevy::prelude::*;

#[derive(Component)]
struct Background;

// Add the animation systems.
pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, init);
}

// Background initialization.
fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create the background.
    let background = asset_server.load("background.png");
    commands.spawn((
        Sprite {
            image: background,
            ..default()
        },
        Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(0.0, 170.0, 0.0)),
        Background,
    ));
}
