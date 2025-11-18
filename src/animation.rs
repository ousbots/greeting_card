use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

#[derive(Component)]
struct AnimationConfig {
    first_index: usize,
    last_index: usize,
    fps: u8,
    frame_timer: Timer,
}

#[derive(Message)]
struct AnimationTrigger {
    flip_x: bool,
}

#[derive(Component)]
struct WalkingSprite;

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_index: first,
            last_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// Add the animation systems to the app.
pub fn add_systems(app: &mut App) {
    app.add_message::<AnimationTrigger>()
        .add_systems(Startup, init)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            (
                send_right.run_if(input_just_pressed(KeyCode::ArrowRight)),
                send_left.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                trigger_animation::<WalkingSprite>,
            ),
        );
}

// Loop through all the sprites and advance their animation, defined by the config.
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // Track how long the current sprite has been displayed.
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            // On last frame, reset to the first, otherwise advance.
            if atlas.index == config.last_index {
                atlas.index = config.first_index;
            } else {
                atlas.index += 1;
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

// Animation initialization.
fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Display help UI in the upper left.
    commands.spawn((
        Text::new("Left: animate left\nRight: animate right"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    // Load the sprite sheet.
    let texture = asset_server.load("man_walking_animation.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 1, None, None);
    let layouts = texture_layouts.add(layout);

    let animation_config = AnimationConfig::new(0, 8, 10);

    // Create the sprite.
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layouts,
                index: animation_config.first_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
        WalkingSprite,
        animation_config,
    ));
}

// Send an animation event when left arrow is pressed.
fn send_left(mut events: MessageWriter<AnimationTrigger>) {
    events.write(AnimationTrigger { flip_x: true });
}

// Send an animation event when right arrow is pressed.
fn send_right(mut events: MessageWriter<AnimationTrigger>) {
    events.write(AnimationTrigger { flip_x: false });
}

// Read animation messages and trigger the animations.
fn trigger_animation<S: Component>(
    mut events: MessageReader<AnimationTrigger>,
    query: Single<(&mut AnimationConfig, &mut Sprite), With<S>>,
) {
    let (mut config, mut sprite) = query.into_inner();
    for event in events.read() {
        sprite.flip_x = event.flip_x;
        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
    }
}
