use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use salva2d::{
    integrations::rapier::{ColliderSampling, FluidsPipeline},
    object::{Boundary, interaction_groups::InteractionGroups},
    rapier::prelude::ColliderHandle,
};

use crate::PIXEL_SIZE;

#[derive(Resource, Deref, DerefMut)]
pub struct Fluids(pub FluidsPipeline);

// These components exist to prevent conflicts with rapiers observers
#[derive(Component)]
pub struct UnregisteredCollider;
#[derive(Component)]
pub struct RegisteredCollider(ColliderHandle);

pub const PARTICLE_RADIUS: f32 = PIXEL_SIZE / 2.0 - 0.1;
impl Default for Fluids {
    fn default() -> Self {
        Fluids(FluidsPipeline::new(PARTICLE_RADIUS, 1.0))
    }
}

pub fn on_collider_add(event: On<Add<Collider>>, mut commands: Commands) {
    commands.entity(event.entity).insert(UnregisteredCollider);
}

pub fn register_colliders(
    mut commands: Commands,
    unregistered_colliders: Query<Entity, With<UnregisteredCollider>>,
    mut fluids: ResMut<Fluids>,
    ctx: ReadRapierContext,
) {
    for collider in unregistered_colliders {
        info!("Registering {collider} to fluid system");
        let Some(co_handle) = ctx
            .single()
            .unwrap()
            .colliders
            .entity2collider()
            .get(&collider)
        else {
            continue;
        };
        let bo_handle = fluids
            .liquid_world
            .add_boundary(Boundary::new(Vec::new(), InteractionGroups::default()));

        fluids.coupling.register_coupling(
            bo_handle,
            *co_handle,
            ColliderSampling::DynamicContactSampling,
        );

        commands
            .entity(collider)
            .remove::<UnregisteredCollider>()
            .insert(RegisteredCollider(*co_handle));
    }
}

pub fn on_collider_remove(
    event: On<Remove<Collider>>,
    mut commands: Commands,
    registered_colliders: Query<&RegisteredCollider>,
    mut fluids: ResMut<Fluids>,
) {
    let Ok(RegisteredCollider(handle)) = registered_colliders.get(event.entity) else {
        return;
    };

    fluids.coupling.unregister_coupling(*handle);
    commands.entity(event.entity).remove::<RegisteredCollider>();
}

pub fn step(
    unregistered_colliders: Query<&UnregisteredCollider>,
    config: Single<&RapierConfiguration>,
    mut fluids: ResMut<Fluids>,
    time: Res<Time>,
    mut ctx: WriteRapierContext,
) {
    if !unregistered_colliders.is_empty() {
        return;
    }

    let mut rapier_context = ctx.single_mut().unwrap();
    fluids.step(
        &config.gravity,
        time.delta_secs(),
        &rapier_context.colliders.colliders,
        &mut rapier_context.rigidbody_set.bodies,
    );
}

#[derive(Component)]
pub struct FluidParticle;
pub fn draw(
    mut commands: Commands,
    particles: Query<Entity, With<FluidParticle>>,
    fluids: ResMut<Fluids>,
) {
    for particle in particles {
        commands.entity(particle).despawn();
    }

    for fluid in fluids.0.liquid_world.fluids().values() {
        for position in &fluid.positions {
            commands.spawn((
                Transform::from_translation(vec3(position.x, position.y, -1.0)),
                Sprite::from_color(Color::linear_rgb(0.0, 0.0, 1.0), Vec2::splat(PIXEL_SIZE)),
                FluidParticle,
            ));
        }
    }
}
