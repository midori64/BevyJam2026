use bevy::prelude::*;
use bevy_rapier2d::dynamics::{Ccd, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties};

use crate::AssetHandles;
use crate::drag::{self, Draggable, on_drag_start};
use crate::screens::Screen;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameplaySet;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup)
        .configure_sets(Update, GameplaySet.run_if(in_state(Screen::Gameplay)))
        .configure_sets(FixedUpdate, GameplaySet.run_if(in_state(Screen::Gameplay)))
        .add_systems(FixedUpdate, drag::handle.in_set(GameplaySet))
        .add_systems(Update, adapt_to_window.in_set(GameplaySet))
        .add_observer(on_drag_start);
}

fn setup(
    mut commands: Commands,
    handles: Res<AssetHandles>,
    meshes: Res<Assets<Mesh>>,
    mut been_ran: Local<bool>,
) {
    if *been_ran {
        return;
    }

    let Some(outline) = meshes.get(handles.vial_outline.id()) else {
        return;
    };

    *been_ran = true;

    // commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(handles.vial.clone()),
        MeshMaterial2d(handles.vial_material.clone()),
        Transform::from_translation(Vec3::Y * 338.0).with_scale(vec3(100.0, 100.0, 1.0)),
        RigidBody::Dynamic,
        Vial,
        Draggable,
        ColliderMassProperties::Density(1.0),
        collider_from_mesh(outline).unwrap(),
        // Ccd::enabled(),
        Velocity::zero(),
    ));

    commands.spawn((
        Sprite::from_color(Color::WHITE, vec2(1.0, 1.0)),
        ColliderMassProperties::Density(10.0),
        Collider::cuboid(0.5, 0.5),
        RigidBody::Fixed,
        Table,
        Ccd::enabled(),
    ));
}

fn collider_from_mesh(mesh: &Mesh) -> Option<Collider> {
    let indices = match mesh.indices()? {
        bevy::mesh::Indices::U32(items) => items.as_chunks::<3>().0.to_vec(),
        _ => unimplemented!(),
    };
    let vertices = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)?
        .as_float3()?
        .iter()
        .map(|array| Vec3::from_array(*array).truncate())
        .collect();
    Collider::trimesh(vertices, indices).ok()
}

fn adapt_to_window(
    mut table_transform: Single<&mut Transform, With<Table>>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Table>)>,
    window: Single<&Window>,
) {
    camera.translation.y = window.height() / 2.0;
    table_transform.scale = vec3(window.width(), 20.0, 1.0);
}

#[derive(Component)]
struct Table;

#[derive(Component)]
struct Vial;
