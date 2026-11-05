use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Component)]
pub struct Draggable;
#[derive(Component)]
pub struct MouseObject;

pub fn on_drag_start(
    event: On<PointerPress>,
    mut commands: Commands,
    draggables: Query<&Transform, With<Draggable>>,
) {
    let draggable_entity = event.entity;
    if event.button == PointerButton::Primary && let Ok(transform) = draggables.get(draggable_entity) {
        let entity_offset = (transform.rotation.inverse() * (event.hit.position.unwrap() - transform.translation)).xy();
        let mouse_entity = commands.spawn((
            Transform::from_translation(event.hit.position.unwrap()),
            RigidBody::Kinematic,
            Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(10.0,10.0)),
            MouseObject,
        )).id();
        commands.spawn((
            avian2d::dynamics::joints::RevoluteJoint::new(draggable_entity, mouse_entity).with_local_anchor1(entity_offset),
            JointForces::new(),
        ));
    }
}

pub fn handle(
    mut commands: Commands,
    // TODO on touch screens may not be single - make a loop later
    dragged: Single<(Entity, &mut LinearVelocity, &mut Transform), With<MouseObject>>,
    camera: Single<(&GlobalTransform, &Camera)>,
    window: Single<&Window>,
    button: Res<ButtonInput<MouseButton>>,
) {
    let (entity, mut vel, pos) = dragged.into_inner();

    if !button.pressed(MouseButton::Left) {
        commands.entity(entity).despawn();
        return;
    }

    if let Some(cursor) = window.cursor_position() && let Ok(position) = camera.1.viewport_to_world_2d(camera.0, cursor) {
        vel.0 = position - pos.translation.xy();
    };
}