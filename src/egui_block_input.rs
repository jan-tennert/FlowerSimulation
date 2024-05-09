use bevy::{input::{ButtonInput, InputSystem}, prelude::{KeyCode, MouseButton, Plugin, Res, ResMut, Resource}};
use bevy::app::{PostUpdate, PreUpdate};
use bevy::prelude::IntoSystemConfigs;
use bevy_egui::{EguiContexts, EguiSet};

//Block input when hovering over egui interfaces

#[derive(Default, Resource)]
struct EguiBlockInputState {
    wants_keyboard_input: bool,
    wants_pointer_input: bool,
}

pub struct BlockInputPlugin;

impl Plugin for BlockInputPlugin {
    
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<EguiBlockInputState>()
        .add_systems(PreUpdate, egui_block_input.after(InputSystem))
        .add_systems(
            PostUpdate,
            egui_wants_input.after(EguiSet::ProcessOutput),
        );
    }
    
}

fn egui_wants_input(mut state: ResMut<EguiBlockInputState>, mut contexts: EguiContexts) {
    state.wants_keyboard_input = contexts.ctx_mut().wants_keyboard_input();
    state.wants_pointer_input = contexts.ctx_mut().wants_pointer_input();
}

fn egui_block_input(
    state: Res<EguiBlockInputState>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut mouse_buttons: ResMut<ButtonInput<MouseButton>>,
) {
    if state.wants_keyboard_input {
        keys.reset_all();
    }
    if state.wants_pointer_input {
        mouse_buttons.reset_all();
    }
}