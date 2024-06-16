use std::{f32::consts::PI, ops::Deref};
use bevy::{ecs::system::RunSystemOnce, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use crate::{constants::PHI, flower::{FlowerSeed, ResetFlowerSeeds, SeedSettings}, petal::{spawn_petals, FlowerPetal, ResetFlowerPetals}, Callback};
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
            FlowerMode::Seed => 6000,
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
    mut seed_settings: ResMut<SeedSettings>,
    mut egui_settings: ResMut<EguiSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UiState>,
    reset_seeds: Query<&Callback, With<ResetFlowerSeeds>>,
    reset_petals: Query<&Callback, With<ResetFlowerPetals>>
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
                    if ui.add(egui::Slider::new(&mut seed_settings.radius, 0.0..=200.0).text("Size")).changed() {
                        changed = true;
                    }
                    if ui.add(egui::Slider::new(&mut seed_settings.amount, 0..=mode.max_amount()).text("Amount")).changed() {
                        changed = true
                    }
                    if mode == FlowerMode::Seed {
                        if ui.checkbox(&mut seed_settings.exp_enabled, "Decreasing distance").changed() {
                            changed = true;
                        }
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
                        ui.label("Object Color");
                        let mut rgb = [seed_settings.color.r(), seed_settings.color.g(), seed_settings.color.b()];
                        if ui.color_edit_button_rgb(&mut rgb).changed() {
                            changed = true;
                        }
                        seed_settings.color = Color::rgb(rgb[0], rgb[1], rgb[2]);
                        seed_settings.material_handle = None;
                    });
                    /*if mode == FlowerMode::Seed {
                        if ui.button("Draw connections").clicked() {
                            commands.run_system(connection_system.single().0);
                        }
                    }*/
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
                    ui.separator();
                    ui.vertical_centered(|ui| {
                        ui.hyperlink_to(
                    egui::RichText::from(
                                "Source by Jan Tennert",
                            )
                            .size(9.), 
                        "https://github.com/jan-tennert/FlowerSimulation");
                    });
            });
        });
    if mode != ui_state.flower_mode {
        ui_state.animate = false;
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
        match ui_state.flower_mode {
            FlowerMode::Seed => {
                seed_settings.mesh_handle = None;
                commands.run_system(reset_seeds.single().0);
            },
            FlowerMode::Petal => {
                seed_settings.mesh_handle = None;
                commands.run_system(reset_petals.single().0);
            }
        }
    }
    
    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Comma) {
        egui_settings.scale_factor *= 1.1;
    } else if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Period) {
        egui_settings.scale_factor *= 0.9;
    }
}