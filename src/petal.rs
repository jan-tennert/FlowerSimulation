use std::{f32::consts::PI, ops::Deref};
use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};

use crate::{constants::PHI, flower::SeedSettings, ui::UiState};

pub struct FlowerPetalPlugin;

impl Plugin for FlowerPetalPlugin {
    fn build(&self, app: &mut App) {
    //    app
           // .init_resource::<SeedSettings>()
         //   .add_systems(Startup, spawn_initial_flowers)
         //   .add_systems(Update, animate_flowers);
    }
}

#[derive(Component)]
pub struct FlowerPetal;

pub fn spawn_petals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    settings: &SeedSettings
) {    
    for i in 1..settings.amount +1 {
        let angle = 2.0 * PI * settings.rotation * (i as f32);
        let x = angle.cos() * settings.distance;
        let y = angle.sin() * settings.distance;
        let rotation = Quat::from_rotation_z(angle + PI / 2.0); // Adjust by 90 degrees (PI/2) to align the petal
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Ellipse::new(5.0 * settings.radius,10.0 * settings.radius))), 
            material: materials.add(settings.color),
            transform: Transform::from_xyz(x, y, 0.0).with_rotation(rotation),
            ..Default::default()
        })
        .insert(FlowerPetal);
    }
}

pub fn clear_petals(
    commands: &mut Commands,
    flowers: Query<Entity, With<FlowerPetal>>
) {
    for entity in flowers.iter() {
        commands.entity(entity).despawn_recursive();
    }
}