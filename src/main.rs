mod setup;
mod flower;
mod constants;
mod ui;
mod egui_block_input;
mod petal;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    }, window::PresentMode,
};
use bevy_egui::EguiPlugin;
use bevy_pancam::PanCamPlugin;
use egui_block_input::BlockInputPlugin;
use flower::FlowerSeedPlugin;
use setup::SetupPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Sunflower Golden Ratio (Jan Tennert)".to_string(),
                    present_mode: PresentMode::AutoVsync,         
                    ..default()
                }),
                ..default()
            }) 
        )   
        .add_plugins(PanCamPlugin::default())
        .add_plugins(UiPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(FlowerSeedPlugin)
        .add_plugins(EguiPlugin)        
        .add_plugins(BlockInputPlugin)
        .run();
}
