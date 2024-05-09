use std::f32::consts::PI;
use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};
use crate::{constants::PHI, flower::{clear_flowers, spawn_flowers, FlowerSeed, NumberSeeds, SeedDistance, SeedRadius, SeedRotation}};
use bevy_egui::{egui::{self, InnerResponse, Response, ScrollArea, Ui}, EguiContexts, EguiSettings};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiState>()
            .add_systems(Update, settings_ui);
    }
}

#[derive(Resource)]
pub struct UiState {
    pub fraction_content: String,
    pub step_size: f32,
    pub animate: bool
}

impl Default for UiState {
    
    fn default() -> Self {
        Self {
            fraction_content: String::from(""),
            step_size: 0.0001,
            animate: false
        }
    }
    
}

pub fn settings_ui(
    mut egui_context: EguiContexts,
    mut seed_rotation: ResMut<SeedRotation>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut num_seeds: ResMut<NumberSeeds>,
    mut seed_radius: ResMut<SeedRadius>,
    mut seed_distance: ResMut<SeedDistance>,   
    flowers: Query<Entity, With<FlowerSeed>>,
    mut egui_settings: ResMut<EguiSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UiState>
) {
    let mut changed = false;
    egui::SidePanel::left("settings_ui")
        // .default_width(250.0)
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .auto_shrink(true)
                .show(ui, |ui| {
                    ui.heading("Settings");
                    ui.label("Rotation per seed");
                    let rotation_slider = ui.add(
                egui::DragValue::new(&mut seed_rotation.0)
                        .speed(ui_state.step_size)         
                        .fixed_decimals(14)
                    );
                    let mut r_changed = rotation_slider.changed();        
                    ui.label("Math input");
                    let fr = ui.text_edit_singleline(&mut ui_state.fraction_content);
                    if fr.lost_focus()  && fr.ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Ok(value) = meval::eval_str(&ui_state.fraction_content) {
                            r_changed = true;
                            seed_rotation.0 = value as f32;
                        }
                    }
                    ui.horizontal(|ui| {
                        if ui.button("1/φ").clicked() {
                            r_changed = true;
                            seed_rotation.0 = 1.0 / PHI;
                        }
                    });
                    ui.label("Animation");
                    ui.add(egui::Slider::new(&mut ui_state.step_size, 0.0..=0.1).drag_value_speed(0.001).logarithmic(true).text("Step size")).changed();
                    let button_text = if ui_state.animate {
                        "Stop animation"
                    } else {
                        "Start animation"
                    };
                    if ui.button(button_text).clicked() {
                        ui_state.animate = !ui_state.animate;  
                    }
                    ui.label("Misc Settings");
                    let d_changed = ui.add(egui::Slider::new(&mut seed_distance.0, 0.0..=30.0).text("Seed density")).changed();
                    let ra_changed = ui.add(egui::Slider::new(&mut seed_radius.0, 0.0..=20.0).text("Seed radius")).changed();
                    let n_changed = ui.add(egui::Slider::new(&mut num_seeds.0, 0..=1000).text("Number seeds")).changed();
                    changed = r_changed || d_changed || ra_changed || n_changed;
            });
        });
    if changed {
        clear_flowers(&mut commands, flowers);
        spawn_flowers(&mut commands, &mut meshes, &mut materials, num_seeds.0, seed_radius.0, seed_distance.0, seed_rotation.0);
    }
    
    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Comma) {
        egui_settings.scale_factor *= 1.1;
    } else if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Period) {
        egui_settings.scale_factor *= 0.9;
    }
}