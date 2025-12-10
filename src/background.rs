use bevy::prelude::*;
use bevy_light_2d::prelude::*;

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
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Background,
    ));

    // Moonlight.
    commands.spawn((
        SpotLight2d {
            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
            intensity: 0.4,
            radius: 200.0,
            direction: 135.0,
            inner_angle: 40.0,
            outer_angle: 60.0,
            source_width: 1.0,
            cast_shadows: true,
            ..default()
        },
        Transform::from_xyz(-160.0, 140.0, 2.0),
    ));
}
