use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Draggable;
#[derive(Component)]
pub struct Dragged {
    hit: Vec2,
}

pub fn on_drag_start(
    event: On<PointerDragStart>,
    mut commands: Commands,
    draggables: Query<&Transform, With<Draggable>>,
) {
    if event.button == PointerButton::Primary
        && let Ok(transform) = draggables.get(event.entity)
    {
        commands.entity(event.entity).insert(Dragged {
            hit: event.hit.position.unwrap().truncate() - transform.translation.truncate(),
        });
    }
}

pub fn handle(
    mut commands: Commands,
    dragged: Single<(
        Entity,
        &Transform,
        &Dragged,
        &mut Velocity,
        &ReadMassProperties,
    )>,
    camera: Single<(&GlobalTransform, &Camera)>,
    window: Single<&Window>,
    button: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let (entity, transform, dragged, mut velocity, properties) = dragged.into_inner();

    if !button.pressed(MouseButton::Left) {
        velocity.linear = Vec2::ZERO;
        commands.entity(entity).remove::<Dragged>();
        return;
    }

    velocity.linear = if let Some(cursor) = window.cursor_position()
        && let Ok(position) = camera.1.viewport_to_world_2d(camera.0, cursor)
    {
        (position - dragged.hit - transform.translation.truncate()
            + properties.local_center_of_mass)
            / time.delta_secs()
    } else {
        Vec2::ZERO
    };
}
