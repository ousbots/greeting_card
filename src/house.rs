use bevy::prelude::*;
use bevy_light_2d::prelude::*;

#[derive(Component)]
struct Background;

// Add the animation systems.
pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, init);
}

// House initialization.
fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create the house.
    let background = asset_server.load("house.png");
    commands.spawn((
        Sprite {
            image: background,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        Background,
    ));

    // Create three (floor is ignored) rectangle occluders to block light from crossing the house boundaries.
    commands.spawn((
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::new(190.0, 2.0),
            },
        },
        Transform::from_xyz(0.0, 10.0, 2.0),
    ));

    commands.spawn((
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::new(2.0, 45.0),
            },
        },
        Transform::from_xyz(-129.0, -35.0, 2.0),
    ));

    commands.spawn((
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::new(2.0, 45.0),
            },
        },
        Transform::from_xyz(138.0, -35.0, 2.0),
    ));
}
