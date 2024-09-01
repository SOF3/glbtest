#![allow(unused_imports)]

use std::f32::consts::FRAC_PI_4;

use bevy::app::{App, AppExit, PluginGroup, Startup};
use bevy::asset::{AssetPlugin, AssetServer, Assets, Handle};
use bevy::color::Color;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::system::{Commands, Res, ResMut};
use bevy::math::primitives::Sphere;
use bevy::math::{Quat, Vec3};
use bevy::pbr::light_consts::lux;
use bevy::pbr::{
    DirectionalLight, DirectionalLightBundle, Material, MaterialMeshBundle, PbrBundle,
    StandardMaterial,
};
use bevy::render::mesh::Mesh;
use bevy::transform::components::Transform;
use bevy::DefaultPlugins;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .run()
}

fn init(mut commands: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh: Handle<Mesh> = assets.load("Cubes.glb#Mesh0");
    let material: Handle<StandardMaterial> = assets.load("material.glb#Material0");
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::Z, Vec3::Y),
        ..<_>::default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: lux::FULL_DAYLIGHT,
            ..<_>::default()
        },
        transform: Transform {
            translation: Vec3::new(0., 1., -1.),
            rotation: Quat::from_rotation_x(-FRAC_PI_4),
            ..<_>::default()
        },
        ..<_>::default()
    });

    commands.spawn(PbrBundle {
        mesh,
        material: material.clone(),
        transform: Transform::from_xyz(0., 1., 5.),
        ..<_>::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(1.)),
        material,
        transform: Transform::from_xyz(0., -1., 5.),
        ..<_>::default()
    });
}
