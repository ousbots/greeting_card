use bevy::prelude::*;

use crate::interaction::{Interactable, State};

#[derive(Component)]
struct Chair;

pub const INTERACTABLE_ID: &str = "chair";

// Add the animation systems.
pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, init);
}

// Animation initialization.
fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create the chair sprite.
    // NOTE: not sure why the chair interaction width is so weird.
    commands.spawn((
        Sprite {
            image: asset_server.load("chair/chair.png"),
            texture_atlas: None,
            ..default()
        },
        Transform::from_xyz(70.0, -58.0, 5.0),
        Chair,
        State::Off,
        Interactable {
            id: INTERACTABLE_ID.to_string(),
            height: 25.0,
            width: 17.0,
            sprite_height: 25.0,
            sprite_width: 25.0,
            ..default()
        },
    ));
}
