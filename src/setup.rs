use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn({
        Camera2dBundle::default()
    });
}