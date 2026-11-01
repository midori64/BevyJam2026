#![feature(iter_array_chunks)]
use bevy::asset::AssetMetaCheck;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::gltf::{GltfMesh, GltfMeshName};
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_obj::ObjPlugin;
use bevy_rapier2d::dynamics::{RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::parry::query;
use bevy_rapier2d::prelude::RapierPhysicsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            FreeCameraPlugin,
            RapierPhysicsPlugin::<()>::default(),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            ObjPlugin,
            MeshPickingPlugin,
        ))
        .init_resource::<AssetHandles>()
        .add_systems(FixedUpdate, (setup, handle_drag))
        .add_observer(on_drag_start)
        // .add_systems(Update, try_spawn_scene)
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

    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(handles.vial.clone()),
        // Sprite::from_color(Color::WHITE, vec2(1.0, 1.0)),
        MeshMaterial2d(handles.vial_material.clone()),
        Transform::default()
            .with_translation(vec3(0.0, 100.0, 0.0))
            .with_scale(vec3(10.0, 10.0, 1.0)),
        RigidBody::Dynamic,
        Collider::trimesh(
            outline
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .unwrap()
                .as_float3()
                .unwrap()
                .into_iter()
                .map(|array| Vec3::from_array(*array).truncate())
                .collect(),
            match outline.indices().unwrap() {
                bevy::mesh::Indices::U32(items) => {
                    items.iter().copied().array_chunks::<3>().collect()
                }
                _ => unimplemented!(),
            },
        )
        .unwrap(),
        Vial,
        Draggable,
        Velocity::zero(),
    ));

    commands.spawn((
        Transform::default()
            .with_translation(vec3(0.0, -100.0, 0.0))
            .with_scale(vec3(100.0, 5.0, 1.0)),
        Sprite::from_color(Color::WHITE, vec2(1.0, 1.0)),
        Collider::cuboid(0.5, 0.5),
        RigidBody::Fixed,
    ));
}

#[derive(Component)]
struct Vial;

#[derive(Component)]
struct Draggable;
#[derive(Component)]
struct Dragged {
    hit: Vec2,
}

fn on_drag_start(
    event: On<PointerDragStart>,
    mut commands: Commands,
    draggables: Query<&Transform, With<Draggable>>,
) {
    if event.button == PointerButton::Primary
        && let Ok(transform) = draggables.get(event.entity)
    {
        commands.entity(event.entity).insert(Dragged {
            hit: dbg!(event.hit.position.unwrap().truncate() - transform.translation.truncate()),
        });
    }
}

fn handle_drag(
    mut commands: Commands,
    dragged: Single<(Entity, &Transform, &Dragged, &mut Velocity)>,
    camera: Single<(&GlobalTransform, &Camera)>,
    window: Single<&Window>,
    button: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let (entity, transform, dragged, mut velocity) = dragged.into_inner();

    if !button.pressed(MouseButton::Left) {
        velocity.linear = Vec2::ZERO;
        commands.entity(entity).remove::<Dragged>();
        return;
    }

    velocity.linear = if let Some(cursor) = window.cursor_position()
        && let Ok(position) = camera.1.viewport_to_world_2d(camera.0, cursor)
    {
        (position - dragged.hit - transform.translation.truncate()) / time.delta_secs()
    } else {
        Vec2::ZERO
    };
}
