use std::{f32::consts::PI, ops::Deref};
use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};

use crate::{constants::PHI, ui::UiState};

pub struct FlowerSeedPlugin;

impl Plugin for FlowerSeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SeedSettings>()
            .add_systems(Startup, spawn_initial_flowers)
            .add_systems(Update, animate_flowers);
    }
}

#[derive(Component)]
pub struct FlowerSeed;

#[derive(Resource)]
pub struct SeedSettings {
    
    pub rotation: f32,
    pub distance: f32,
    pub radius: f32,
    pub amount: i32,
    pub color: Color,
    
}

impl Default for SeedSettings {
    
    fn default() -> Self {
        Self { rotation: 0., distance: 4.0, radius: 4.0, amount: 50, color: Color::ORANGE}
    }
    
}

impl SeedSettings {
        
    pub fn default_petal() -> Self {
        Self { rotation: 0., distance: 50.0, radius: 4.0, amount: 1, color: Color::ORANGE}        
    }
    
}

fn spawn_initial_flowers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    seed_settings: Res<SeedSettings>
) {
    spawn_flowers(&mut commands, &mut meshes, &mut materials, seed_settings.deref());
}

fn animate_flowers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut seed_settings: ResMut<SeedSettings>,
    ui_state: Res<UiState>,
    time: Res<Time>,  
    flowers: Query<Entity, With<FlowerSeed>>,
) {
    if !ui_state.animate {
        return;
    }
    seed_settings.rotation = seed_settings.rotation + (ui_state.step_size * time.delta_seconds());
    clear_flowers(&mut commands, flowers);
    spawn_flowers(&mut commands, &mut meshes, &mut materials, seed_settings.deref());
}

pub fn spawn_flowers(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    settings: &SeedSettings
) {    

    for i in 1..settings.amount+1  {
        let angle = 2.0 * PI * settings.rotation * (i as f32);
        let radius = 5.0 * (i as f32).sqrt(); 
        let x = angle.cos() * radius * settings.distance; 
        let y = angle.sin() * radius * settings.distance;
    
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: settings.radius })), 
            material: materials.add(settings.color),
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(FlowerSeed);
    }
}

pub fn clear_flowers(
    commands: &mut Commands,
    flowers: Query<Entity, With<FlowerSeed>>
) {
    for entity in flowers.iter() {
        commands.entity(entity).despawn_recursive();
    }
}