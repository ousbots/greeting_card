use bevy::prelude::*;

// Added to Interactable entities when they should be highlighted.
#[derive(Component)]
pub struct Highlight {
    pub elapsed_offset: f32,
    pub size: Vec2,
}

// Add to entities that can initiate interactions.
#[derive(Component)]
pub struct Interactor {
    pub width: f32,
    pub height: f32,
}

// Add to entities that can be interacted with.
#[derive(Component, Default)]
pub struct Interactable {
    pub id: String,
    pub height: f32,
    pub width: f32,
    pub sprite_height: f32,
    pub sprite_width: f32,
    pub highlighted: bool,
}

// Added to Interactor entities when they're in range of an Interactable.
#[derive(Component)]
pub struct InRange {
    pub id: String,
}

// Message sent when an interaction is triggered.
#[derive(Message)]
pub struct InteractionEvent {
    pub id: String,
}

#[derive(Clone, Component, Copy, PartialEq, Eq)]
pub enum State {
    Off,
    On,
}

// Add the interaction systems.
pub fn add_systems(app: &mut App) {
    app.add_message::<InteractionEvent>().add_systems(
        Update,
        (
            detect_overlaps,
            handle_highlight,
            handle_highlight_reset,
            handle_highlight_state_change,
        ),
    );
}

// Axis-Aligned Bounding Box overlap detection.
pub fn aabb_overlap(pos_1: Vec2, width_1: f32, height_1: f32, pos_2: Vec2, width_2: f32, height_2: f32) -> bool {
    let half_width_1 = width_1 / 2.0;
    let half_height_1 = height_1 / 2.0;
    let half_width_2 = width_2 / 2.0;
    let half_height_2 = height_2 / 2.0;

    let left_1 = pos_1.x - half_width_1;
    let right_1 = pos_1.x + half_width_1;
    let top_1 = pos_1.y + half_height_1;
    let bottom_1 = pos_1.y - half_height_1;

    let left_2 = pos_2.x - half_width_2;
    let right_2 = pos_2.x + half_width_2;
    let top_2 = pos_2.y + half_height_2;
    let bottom_2 = pos_2.y - half_height_2;

    !(right_1 < left_2 || left_1 > right_2 || top_1 < bottom_2 || bottom_1 > top_2)
}

// Detects overlaps between Interactors and Interactables.
fn detect_overlaps(
    time: Res<Time>,
    mut commands: Commands,
    mut interactables: Query<(Entity, &State, &GlobalTransform, &mut Interactable)>,
    interactors: Query<(&GlobalTransform, &Interactor)>,
    in_range: Query<(Entity, &InRange)>,
) {
    for (interactable_entity, interactable_state, interactable_transform, interactable) in &mut interactables {
        for (interactor_transform, interactor) in &interactors {
            let overlaping = aabb_overlap(
                interactor_transform.translation().truncate(),
                interactor.width,
                interactor.height,
                interactable_transform.translation().truncate(),
                interactable.width,
                interactable.height,
            );

            // Update InRange component based on overlap.
            let currently_in_range = in_range
                .iter()
                .find(|(e, _)| *e == interactable_entity)
                .map(|(_, r)| r.id.clone());

            match (currently_in_range, overlaping) {
                // New entity entered in-range.
                (None, true) => {
                    commands.entity(interactable_entity).insert(InRange {
                        id: interactable.id.clone(),
                    });
                    if !interactable.highlighted && *interactable_state == State::Off {
                        commands.entity(interactable_entity).insert(Highlight {
                            elapsed_offset: time.elapsed_secs(),
                            size: Vec2::new(interactable.sprite_width, interactable.sprite_height),
                        });
                    }
                }

                // Entity in-range changed.
                (Some(current_id), true) if current_id != interactable.id => {
                    commands.entity(interactable_entity).insert(InRange {
                        id: interactable.id.clone(),
                    });
                }

                // Entity left in-range.
                (Some(_), false) => {
                    commands.entity(interactable_entity).remove::<InRange>();
                    commands.entity(interactable_entity).remove::<Highlight>();
                }

                _ => {}
            }
        }
    }
}

// Apply a pulsing scale effect to highlighted sprites using custom_size.
fn handle_highlight(time: Res<Time>, mut query: Query<(&mut Sprite, &Highlight)>) {
    for (mut sprite, highlight) in &mut query {
        let pulse = (((time.elapsed_secs() - highlight.elapsed_offset) * 4.).sin() + 1.).mul_add(0.1, 1.);
        sprite.color = Color::srgba(pulse, pulse, pulse, 1.);
        let scale_factor = ((pulse - 1.) / 4.) + 1.;
        sprite.custom_size = Some(highlight.size * scale_factor);
    }
}

// Reset sprite color and custom_size when highlight is removed.
fn handle_highlight_reset(mut removed: RemovedComponents<Highlight>, mut query: Query<&mut Sprite>) {
    for entity in removed.read() {
        if let Ok(mut sprite) = query.get_mut(entity) {
            sprite.color = Color::WHITE;
            sprite.custom_size = None;
        }
    }
}

// Remove highlight component when an interactable is turned on.
fn handle_highlight_state_change(
    mut commands: Commands,
    query: Query<(Entity, &mut Interactable, &State), (With<Highlight>, Changed<State>)>,
) {
    for (entity, mut interactable, state) in query {
        if *state == State::On {
            commands.entity(entity).remove::<Highlight>();
            interactable.highlighted = true;
        }
    }
}
