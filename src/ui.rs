use std::f32::consts::PI;
use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};
use crate::{constants::PHI, flower::{clear_flowers, spawn_flowers, FlowerSeed, NumberSeeds, SeedDistance, SeedRadius, SeedRotation}};
use bevy_egui::{egui::{self, InnerResponse, Response, ScrollArea, Ui}, EguiContexts, EguiSettings};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, settings_ui);
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
                    let mut r_changed = ui.add(egui::Slider::new(&mut seed_rotation.0, 0.0..=1.0).step_by(0.00001).drag_value_speed(0.0001).text("Rotation per seed")).changed();
                    ui.horizontal(|ui| {
                        if ui.button("1/PI").clicked() {
                            r_changed = true;
                            seed_rotation.0 = 1.0 / PI;
                        }
                        if ui.button("1/PHI").clicked() {
                            r_changed = true;
                            seed_rotation.0 = 1.0 / PHI;
                        }
                    });
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