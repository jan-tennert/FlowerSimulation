use std::{f32::consts::PI, ops::Deref};
use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};
use crate::{constants::PHI, flower::{clear_flowers, spawn_flowers, FlowerSeed, SeedSettings}, petal::{clear_petals, spawn_petals, FlowerPetal}};
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
    pub animate: bool,
    pub flower_mode: FlowerMode
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FlowerMode {
    Seed,
    Petal,
}

impl FlowerMode {
    
    fn max_density(self) -> f32 {
        match self {
            FlowerMode::Seed => 30.0,
            FlowerMode::Petal => 100.0
        }
    }
    
    fn max_amount(self) -> i32 {
        match self {
            FlowerMode::Seed => 1000,
            FlowerMode::Petal => 40
        }
    }
    
}


impl Default for UiState {
    
    fn default() -> Self {
        Self {
            fraction_content: String::from(""),
            step_size: 0.0001,
            animate: false,
            flower_mode: FlowerMode::Seed
        }
    }
    
}

pub fn settings_ui(
    mut egui_context: EguiContexts,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut seed_settings: ResMut<SeedSettings>,
    flowers: Query<Entity, With<FlowerSeed>>,
    petals: Query<Entity, With<FlowerPetal>>,
    mut egui_settings: ResMut<EguiSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UiState>
) {
    let mut changed = false;
    let mode = ui_state.flower_mode.clone();    
    egui::SidePanel::left("settings_ui")
        // .default_width(250.0)
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .auto_shrink(true)
                .show(ui, |ui| {
                    ui.heading("Settings");
                    egui::ComboBox::from_label("Flower mode")
                    .selected_text(format!("{mode:?}"))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(&mut ui_state.flower_mode, FlowerMode::Seed, "Seed");
                        ui.selectable_value(&mut ui_state.flower_mode, FlowerMode::Petal, "Petal");
                    });
                    ui.label("Rotation");
                    if ui.add(
                egui::DragValue::new(&mut seed_settings.rotation)
                        .speed(ui_state.step_size)         
                        .fixed_decimals(14)
                    ).changed() {
                        changed = true;
                    }
                    ui.label("Math input");
                    let fr = ui.text_edit_singleline(&mut ui_state.fraction_content);
                    if fr.lost_focus()  && fr.ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Ok(value) = meval::eval_str(&ui_state.fraction_content) {
                            changed = true;
                            seed_settings.rotation = value as f32;
                        }
                    }
                    ui.horizontal(|ui| {
                        if ui.button("1/φ").clicked() {
                            changed = true;
                            seed_settings.rotation = 1.0 / PHI;
                        }
                    });
                    if mode == FlowerMode::Seed {
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
                    }
                    ui.label("Misc Settings");
                    if ui.add(egui::Slider::new(&mut seed_settings.distance, 0.0..=mode.max_density()).text("Density")).changed() {
                        changed = true
                    }
                    if ui.add(egui::Slider::new(&mut seed_settings.radius, 0.0..=20.0).text("Size")).changed() {
                        changed = true;
                    }
                    if ui.add(egui::Slider::new(&mut seed_settings.amount, 0..=mode.max_amount()).text("Amount")).changed() {
                        changed = true
                    }
                    if mode == FlowerMode::Petal {
                        ui.horizontal(|ui| {
                            if ui.button("-1").clicked() {
                                changed = true;
                                seed_settings.amount = i32::max(seed_settings.amount - 1 , 1);
                            }
                            if ui.button("+1").clicked() {
                                changed = true;
                                seed_settings.amount = i32::min(seed_settings.amount + 1 , mode.max_amount());
                            }
                        });
                    }
                    ui.horizontal(|ui| {
                        ui.label("Seed Color");
                        let mut rgb = [seed_settings.color.r(), seed_settings.color.g(), seed_settings.color.b()];
                        if ui.color_edit_button_rgb(&mut rgb).changed() {
                            changed = true;
                        }
                        seed_settings.color = Color::rgb(rgb[0], rgb[1], rgb[2]);
                    });
                    if ui.button("Reset Settings").clicked() {
                        match mode.clone() {
                            FlowerMode::Seed => {
                                *seed_settings = SeedSettings::default();
                            },
                            FlowerMode::Petal => {
                                *seed_settings = SeedSettings::default_petal();
                            }
                        }
                        changed = true;
                    }                
            });
        });
    if mode != ui_state.flower_mode {
        match ui_state.flower_mode.clone() {
            FlowerMode::Seed => {
                *seed_settings = SeedSettings::default();
            },
            FlowerMode::Petal => {
                *seed_settings = SeedSettings::default_petal();
            }
        }
    }    
    if changed || mode != ui_state.flower_mode {
        clear_flowers(&mut commands, flowers);
        clear_petals(&mut commands, petals);
        match ui_state.flower_mode {
            FlowerMode::Seed => {
                spawn_flowers(&mut commands, &mut meshes, &mut materials, seed_settings.deref());
            },
            FlowerMode::Petal => {
                spawn_petals(&mut commands, &mut meshes, &mut materials, seed_settings.deref());
            }
        }
    }
    
    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Comma) {
        egui_settings.scale_factor *= 1.1;
    } else if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Period) {
        egui_settings.scale_factor *= 0.9;
    }
}