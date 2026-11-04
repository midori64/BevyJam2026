// #![feature(iter_array_chunks)]

mod drag;
mod fluids;

use bevy::asset::AssetMetaCheck;
use bevy::camera_controller::free_camera::{FreeCameraPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_obj::ObjPlugin;

use bevy_rapier2d::prelude::*;

use crate::drag::{Draggable, on_drag_start};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            FreeCameraPlugin,
            RapierPhysicsPlugin::<()>::default().with_custom_initialization(
                RapierContextInitialization::InitializeDefaultRapierContext {
                    rapier_configuration: RapierConfiguration {
                        gravity: Vec2::new(0.0, -200.0),
                        ..RapierConfiguration::new(1.0)
                    },
                    integration_parameters: IntegrationParameters {
                        // num_solver_iterations: 128,
                        ..Default::default()
                    },
                    broad_phase_optimization_strategy:
                        BroadPhaseOptimizationStrategy::SubtreeOptimizer,
                },
            ),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            ObjPlugin,
            MeshPickingPlugin,
        ))
        .init_resource::<AssetHandles>()
        .add_systems(Update, (try_setup, adapt_to_window))
        .add_systems(
            FixedUpdate,
            drag::handle
        )
        .add_observer(on_drag_start)
        .run();
}

#[derive(Resource)]
struct AssetHandles {
    vial: Handle<Mesh>,
    vial_outline: Handle<Mesh>,
    vial_material: Handle<ColorMaterial>,
}

impl FromWorld for AssetHandles {
    fn from_world(world: &mut World) -> Self {
        Self {
            vial: world.load_asset("vial.obj"),
            vial_outline: world.load_asset("vial-outline.obj"),
            vial_material: world.add_asset(ColorMaterial::from_color(Color::WHITE.with_alpha(0.1))),
        }
    }
}

fn try_setup(
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

    commands.spawn(Camera2d);

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

fn adapt_to_window(
    mut table_transform: Single<&mut Transform, With<Table>>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Table>)>,
    window: Single<&Window>,
) {
    camera.translation.y = window.height() / 2.0;
    table_transform.scale = vec3(window.width(), 20.0, 1.0);
}

fn collider_from_mesh(mesh: &Mesh) -> Option<Collider> {
    let indices = match mesh.indices()? {
        bevy::mesh::Indices::U32(items) => items.chunks_exact(3).map(|chunk| { chunk.try_into().unwrap() }).collect(),
        _ => unimplemented!(),
    };
    let vertices = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)?
        .as_float3()?
        .into_iter()
        .map(|array| Vec3::from_array(*array).truncate())
        .collect();
    Collider::trimesh(vertices, indices).ok()
}

#[derive(Component)]
struct Vial;

#[derive(Component)]
struct Table;
