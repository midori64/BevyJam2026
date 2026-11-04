// use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

// #[derive(Component)]
// pub struct Draggable;
// // #[derive(Component)]
// // pub struct Dragged;
// #[derive(Component)]
// pub struct MouseObject;

// pub fn on_drag_start(
//     event: On<PointerPress>,
//     mut commands: Commands,
//     draggables: Query<&Transform, With<Draggable>>,
// ) {
//     let entity = event.entity;
//     if event.button == PointerButton::Primary && let Ok(transform) = draggables.get(entity) {
//         let entity_offset = (transform.rotation.inverse() * (event.hit.position.unwrap() - transform.translation)).xy();
//         // let joint = bevy_rapier2d::dynamics::SpringJointBuilder::new(0.0, 10000000.0, 100.0).local_anchor1(entity_offset).local_anchor2(Vec2::new(0.0,0.0));
//         // let joint2 = bevy_rapier2d::dynamics::RevoluteJointBuilder::new().local_anchor1(entity_offset).local_anchor2(Vec2::new(0.0,0.0)).build();
//         let joint3 = bevy_rapier2d::dynamics::RopeJointBuilder::new(0.1).local_anchor1(entity_offset).local_anchor2(Vec2::new(0.0,0.0)).build();
//         commands.spawn((
//                 Transform::from_translation(event.hit.position.unwrap()),
//                 // RigidBody::KinematicPositionBased,
//                 RigidBody::KinematicVelocityBased,
//                 Velocity::zero(),
//                 MouseObject,
//                 Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(10.0,10.0)),
//                 bevy_rapier2d::dynamics::ImpulseJoint::new(entity, joint3)
//         ));
//         // commands.entity(entity).insert(Dragged);
//     }
// }

// pub fn handle(
//     mut commands: Commands,
//     // TODO on touch screens may not be single - make a loop later
//     dragged: Single<(Entity, &mut Velocity, &mut Transform), With<MouseObject>>,
//     camera: Single<(&GlobalTransform, &Camera)>,
//     window: Single<&Window>,
//     button: Res<ButtonInput<MouseButton>>,
// ) {
//     let (entity, mut vel, pos) = dragged.into_inner();

//     if !button.pressed(MouseButton::Left) {
//         commands.entity(entity).despawn();
//         return;
//     }

//     if let Some(cursor) = window.cursor_position() && let Ok(position) = camera.1.viewport_to_world_2d(camera.0, cursor) {
//         vel.linear = position - pos.translation.xy();
//     };
// }















use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Draggable;
#[derive(Component)]
pub struct Dragged {
    grab_local_offset: Vec2,
}

pub fn on_drag_start(
    event: On<PointerDragStart>,
    mut commands: Commands,
    draggables: Query<&Transform, With<Draggable>>,
) {
    if event.button == PointerButton::Primary
    && let Ok(transform) = draggables.get(event.entity)
    {
        let entity_offset = (transform.rotation.inverse() * (event.hit.position.unwrap() - transform.translation)).xy();
        commands.entity(event.entity).insert((LockedAxes::ROTATION_LOCKED, Dragged {
            grab_local_offset: entity_offset
        }));
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
        commands.entity(entity).remove::<LockedAxes>();
        return;
    }

    velocity.linear = if let Some(cursor) = window.cursor_position()
        && let Ok(position) = camera.1.viewport_to_world_2d(camera.0, cursor)
    {
        (position - dragged.grab_local_offset - transform.translation.truncate()
            + properties.local_center_of_mass)
            / time.delta_secs()
    } else {
        Vec2::ZERO
    };
}
