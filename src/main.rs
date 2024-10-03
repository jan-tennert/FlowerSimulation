#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod constants;
mod egui_block_input;
mod flower;
mod petal;
mod setup;
mod ui;

use bevy::{ecs::system::SystemId, prelude::*, window::PresentMode};
use bevy_egui::EguiPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_vector_shapes::Shape2dPlugin;
use egui_block_input::BlockInputPlugin;
use flower::FlowerSeedPlugin;
use petal::FlowerPetalPlugin;
use setup::SetupPlugin;
use ui::UiPlugin;

#[derive(Component)]
pub struct Callback(SystemId);

#[derive(Component)]
pub struct FlowerComponent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sunflower Golden Ratio (Jan Tennert)".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Shape2dPlugin::default())
        //  .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PanCamPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(FlowerPetalPlugin)
        .add_plugins(FlowerSeedPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(BlockInputPlugin)
        .run();
}
