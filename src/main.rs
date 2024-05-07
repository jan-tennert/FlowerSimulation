mod setup;
mod flower;
mod constants;
mod ui;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    }, window::PresentMode,
};
use bevy_egui::EguiPlugin;
use flower::FlowerPlugin;
use setup::SetupPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flower (Jan Tennert)".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }) 
        )   
        .add_plugins(EguiPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(FlowerPlugin)
        .run();
}
