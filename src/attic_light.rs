use bevy::{audio::Volume, prelude::*};
use bevy_light_2d::prelude::*;
use rand::Rng;

use crate::{
    flickering_light::FlickeringLight,
    interaction::{InRange, Interactable, InteractionEvent, State},
};

#[derive(Clone, Resource)]
struct AudioAssets {
    on: Handle<AudioSource>,
    off: Handle<AudioSource>,
}

#[derive(Clone, Resource)]
struct SpriteAssets {
    on: Handle<Image>,
    off: Handle<Image>,
}

#[derive(Component)]
struct AtticLight;

const INTERACTABLE_ID: &str = "attic-light";

const SWITCH_VOLUME: f32 = 0.40;

// Light effect colors.
const LIGHT_COLORS: [Color; 3] = [
    Color::srgb(1.0, 0.6, 0.2),
    Color::srgb(1.0, 0.7, 0.1),
    Color::srgb(1.0, 0.5, 0.3),
];

// Add the animation systems.
pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, init).add_systems(
        Update,
        (
            handle_interaction,
            handle_light.in_set(crate::flickering_light::LightInsertionSet),
        ),
    );
}

// Listen for interaction events and update the state.
fn handle_interaction(mut events: MessageReader<InteractionEvent>, mut query: Query<&mut State, With<AtticLight>>) {
    for event in events.read() {
        if event.id == INTERACTABLE_ID
            && let Ok(mut state) = query.single_mut()
        {
            match *state {
                State::Off => {
                    *state = State::On;
                }

                State::On => {
                    *state = State::Off;
                }
            }
        }
    }
}

// Add or remove flickering light based on the fireplace state.
fn handle_light(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    sprite_assets: Res<SpriteAssets>,
    parent_query: Query<(&Children, &State, &mut Sprite), (With<AtticLight>, With<InRange>, Changed<State>)>,
    mut light_query: Query<(Entity, &mut PointLight2d)>,
) {
    let mut rng = rand::rng();

    // Find the child light entity.
    for (children, state, mut sprite) in parent_query {
        for child in children.iter() {
            if let Ok((entity, mut light)) = light_query.get_mut(child) {
                match *state {
                    State::On => {
                        sprite.image = sprite_assets.on.clone();

                        commands.spawn((
                            AudioPlayer::new(audio_assets.on.clone()),
                            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(SWITCH_VOLUME)),
                        ));

                        commands.entity(entity).insert(FlickeringLight {
                            seed: rng.random_range(0.0..1000.0),
                            intensity_amplitude: 0.2,
                            intensity_frequency: 2.0,
                            intensity_min: 0.4,
                            intensity_octaves: 4,
                            color_frequency: 100.0,
                            color_octaves: 5,
                            color_seed_offset: 100.0,
                            color_temperature: 0.5,
                            colors: LIGHT_COLORS.to_vec(),
                            time_offset: rng.random_range(0.0..100.0),
                        });
                    }
                    State::Off => {
                        sprite.image = sprite_assets.off.clone();

                        commands.spawn((
                            AudioPlayer::new(audio_assets.off.clone()),
                            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(SWITCH_VOLUME)),
                        ));

                        commands.entity(entity).remove::<FlickeringLight>();
                        light.intensity = 0.0;
                    }
                }
            }
        }
    }
}

// Attic light initialization.
fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the sprite sheets.
    let sprites = SpriteAssets {
        on: asset_server.load("house/light_switch_on.png"),
        off: asset_server.load("house/light_switch_off.png"),
    };
    commands.insert_resource(sprites.clone());

    let audio = AudioAssets {
        on: asset_server.load("house/light_switch_on.ogg"),
        off: asset_server.load("house/light_switch_off.ogg"),
    };
    commands.insert_resource(audio);

    // Parent position is the hidden switch.
    let parent = commands
        .spawn((
            AtticLight,
            State::Off,
            Sprite {
                image: sprites.off,
                ..default()
            },
            Transform::from_xyz(148.0, -50.0, 5.0),
            Interactable {
                id: INTERACTABLE_ID.to_string(),
                height: 4.0,
                width: 3.0,
                ..default()
            },
        ))
        .id();

    // Spawn light, Local offset from switch (-21, 110, 0) → Global position (128, 60, 5)
    let light = commands
        .spawn((
            Transform::from_xyz(-20.0, 110.0, 0.0),
            PointLight2d {
                color: LIGHT_COLORS[0],
                intensity: 0.0,
                radius: 160.0,
                cast_shadows: true,
                ..default()
            },
        ))
        .id();
    commands.entity(parent).add_child(light);
}
