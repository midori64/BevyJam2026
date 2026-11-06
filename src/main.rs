// #![feature(iter_array_chunks)]

mod drag;
mod fluids;
mod gameplay;
mod main_menu;
mod screens;

use bevy::asset::AssetMetaCheck;
use bevy::camera_controller::free_camera::FreeCameraPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_obj::ObjPlugin;

use bevy_rapier2d::prelude::*;

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
        .add_plugins((screens::plugin, main_menu::plugin, gameplay::plugin))
        .add_systems(Startup, setup)
        .init_resource::<AssetHandles>()
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
