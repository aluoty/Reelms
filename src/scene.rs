use bevy::prelude::*;
use bevy::sprite::{ColorMaterial, MeshMaterial2d};

use crate::models::location_theme;
use crate::world::GameWorld;

#[derive(Component)]
pub struct WorldSceneRoot;

#[derive(Component)]
pub struct SkyBackdrop;

#[derive(Component)]
pub struct SceneryMesh;

#[derive(Resource, Default)]
pub struct SceneState {
    pub location: String,
    pub weather: String,
    pub moon: String,
}

pub fn setup_world_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world: Res<GameWorld>,
    mut state: ResMut<SceneState>,
) {
    spawn_scene(&mut commands, &mut meshes, &mut materials, &world);

    state.location = world.current_location.clone();
    state.weather = world.weather.clone();
    state.moon = world.moon_phase.clone();
}

pub fn refresh_world_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world: Res<GameWorld>,
    mut state: ResMut<SceneState>,
    root_q: Query<Entity, With<WorldSceneRoot>>,
    children_q: Query<&Children>,
) {
    if state.location == world.current_location
        && state.weather == world.weather
        && state.moon == world.moon_phase
    {
        return;
    }

    for root in root_q.iter() {
        if let Ok(children) = children_q.get(root) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
        commands.entity(root).despawn();
    }

    spawn_scene(&mut commands, &mut meshes, &mut materials, &world);

    state.location = world.current_location.clone();
    state.weather = world.weather.clone();
    state.moon = world.moon_phase.clone();
}

fn spawn_scene(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    world: &GameWorld,
) {
    let theme = location_theme(
        &world.current_location,
        &world.weather,
        &world.moon_phase,
    );

    commands
        .spawn((
            WorldSceneRoot,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                SkyBackdrop,
                Mesh2d(meshes.add(Rectangle::new(1020.0, 620.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(theme.sky))),
                Transform::from_xyz(0.0, 0.0, -20.0),
            ));

            for piece in theme.pieces {
                let mesh_handle = meshes.add(piece.mesh);
                let mut transform = Transform::from_translation(piece.translation);
                transform.rotation = Quat::from_rotation_z(piece.rotation);
                transform.scale = piece.scale;

                parent.spawn((
                    SceneryMesh,
                    Mesh2d(mesh_handle),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(piece.color))),
                    transform,
                ));
            }
        });
}

pub fn animate_water_shimmer(
    time: Res<Time>,
    world: Res<GameWorld>,
    mut scenery_q: Query<&mut Transform, With<SceneryMesh>>,
) {
    let t = time.elapsed_secs();
    let shimmer = (t * 0.6).sin() * 0.8;
    for mut transform in scenery_q.iter_mut() {
        if transform.translation.y < -100.0 && transform.translation.y > -200.0 {
            transform.translation.y += shimmer * 0.02;
        }
    }
    let _ = &world.current_location;
}
