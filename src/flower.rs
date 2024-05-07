use std::f32::consts::PI;
use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}};

use crate::constants::PHI;

pub struct FlowerPlugin;

impl Plugin for FlowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NumberSeeds(50))
            .insert_resource(SeedRadius(4.0))
            .insert_resource(SeedDistance(4.0))
            .insert_resource(SeedRotation(1.0 / PHI))
            .add_systems(Startup, spawn_initial_flowers);
    }
}

#[derive(Component)]
pub struct FlowerSeed;

#[derive(Resource)]
pub struct NumberSeeds(pub i32);
#[derive(Resource)]
pub struct SeedRadius(pub f32);
#[derive(Resource)]
pub struct SeedDistance(pub f32);
#[derive(Resource)]
pub struct SeedRotation(pub f32);

fn spawn_initial_flowers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    num_seeds: Res<NumberSeeds>,
    seed_radius: Res<SeedRadius>,
    seed_distance: Res<SeedDistance>,
    seed_rotation: Res<SeedRotation>
) {
    spawn_flowers(&mut commands, &mut meshes, &mut materials, num_seeds.0, seed_radius.0, seed_distance.0, seed_rotation.0);
}

pub fn spawn_flowers(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    num_seeds: i32,
    seed_radius: f32,
    seed_distance: f32,
    seed_rotation: f32
) {
    // Golden ratio
    
    // Starting angle
    let start_angle = 0.0;

    for i in 1..num_seeds {
        let angle = start_angle + ((2.0 * PI)/ (1. / seed_rotation)) * (i as f32);
        let radius = 5.0 * (i as f32).sqrt(); // Adjust radius based on index
        let x = angle.cos() * radius * seed_distance; // Multiply by 2.0 to space them out more
        let y = angle.sin() * radius * seed_distance;
    
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: seed_radius })), // Use smaller radius for points
            material: materials.add(Color::ORANGE),
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