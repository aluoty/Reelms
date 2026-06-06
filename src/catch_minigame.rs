use bevy::prelude::*;
use bevy::sprite::{ColorMaterial, MeshMaterial2d};
use rand::Rng;

use crate::fish::Fish;
use crate::fish_engine::rarity_rank;
use crate::models::{
    build_fish_part_meshes, build_rod_meshes, fish_model_for_species, palette, rarity_glow_color,
};
use crate::world::GameWorld;

#[derive(Resource)]
pub struct CatchMinigame {
    pub fish: Fish,
    pub effective_difficulty: f64,
    pub marker_position: f64,
    pub marker_direction: f64,
    pub marker_speed: f64,
    pub green_width: f64,
    pub yellow_width: f64,
    pub pull_count: i32,
    pub catch_progress: i32,
    pub finished: bool,
    pub caught: bool,
    pub pull_frames: i32,
    pub fail_frames: i32,
    pub tick: i32,
}

impl CatchMinigame {
    pub fn new(fish: Fish, rod_level: i32, rod_reduction: f64) -> Self {
        let fish_difficulty = rarity_rank(&fish.rarity);
        let base_difficulty = 0.9 + fish_difficulty as f64 * 0.35;
        let effective_difficulty = (base_difficulty - rod_reduction).max(0.75);
        let green_width = (0.30 - effective_difficulty * 0.06).max(0.14);
        let yellow_width = (0.36 - effective_difficulty * 0.05).max(0.16);
        let marker_speed = (0.011 + effective_difficulty * 0.0038
            + (rod_level - 5).max(0) as f64 * 0.0015)
            .min(0.030);

        Self {
            fish,
            effective_difficulty,
            marker_position: 0.0,
            marker_direction: 1.0,
            marker_speed,
            green_width,
            yellow_width,
            pull_count: 0,
            catch_progress: 45,
            finished: false,
            caught: false,
            pull_frames: 0,
            fail_frames: 0,
            tick: 0,
        }
    }

    pub fn resolve_pull(&mut self, rng: &mut impl Rng) {
        if self.finished {
            return;
        }
        self.pull_count += 1;
        self.pull_frames = 10;

        let zone_score = zone_score(
            self.marker_position,
            self.effective_difficulty,
            rng,
        );
        let fish_difficulty = rarity_rank(&self.fish.rarity);
        let delta = (zone_score * (17.0 - fish_difficulty as f64 * 1.2)).round() as i32;
        self.catch_progress = (self.catch_progress + delta).clamp(0, 100);

        if self.catch_progress >= 100 {
            self.finished = true;
            self.caught = true;
        } else if self.catch_progress <= 0 {
            self.finished = true;
            self.caught = false;
            self.fail_frames = 16;
        }
    }

    pub fn tick_marker(&mut self, dt: f32) {
        if self.finished {
            return;
        }
        self.tick += 1;
        if self.pull_frames > 0 {
            self.pull_frames -= 1;
        }
        if self.fail_frames > 0 {
            self.fail_frames -= 1;
        }

        self.marker_position += self.marker_direction * self.marker_speed * dt as f64 * 60.0;
        if self.marker_position >= 1.0 {
            self.marker_position = 1.0;
            self.marker_direction = -1.0;
        } else if self.marker_position <= 0.0 {
            self.marker_position = 0.0;
            self.marker_direction = 1.0;
        }
    }
}

fn zone_score(marker_pos: f64, effective_difficulty: f64, rng: &mut impl Rng) -> f64 {
    let green_width = (0.30 - effective_difficulty * 0.06).max(0.14);
    let yellow_width = (0.36 - effective_difficulty * 0.05).max(0.16);
    let green_start = 0.5 - green_width / 2.0;
    let green_end = 0.5 + green_width / 2.0;
    let yellow_start = 0.5 - yellow_width / 2.0;
    let yellow_end = 0.5 + yellow_width / 2.0;

    if marker_pos >= green_start && marker_pos <= green_end {
        1.0
    } else if marker_pos >= yellow_start && marker_pos <= yellow_end {
        if rng.random_bool(0.62) {
            0.4
        } else {
            -0.2
        }
    } else {
        -1.0
    }
}

#[derive(Component)]
pub struct CatchMinigameRoot;

#[derive(Component)]
pub struct CatchBarRoot;

#[derive(Component)]
pub struct CatchMarker;

#[derive(Component)]
pub struct CatchGreenZone;

#[derive(Component)]
pub struct CatchYellowZone;

#[derive(Component)]
pub struct CatchFishSprite;

#[derive(Component)]
pub struct CatchRodSprite;

#[derive(Component)]
pub struct CatchWaterWorld;

pub fn setup_catch_minigame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world: Res<GameWorld>,
    mut next_state: ResMut<NextState<crate::GameScreen>>,
) {
    let Some(fish) = world.pending_fish.clone() else {
        next_state.set(crate::GameScreen::Main);
        return;
    };

    let minigame = CatchMinigame::new(
        fish.clone(),
        world.rod_level,
        world.rod_catch_difficulty_reduction(),
    );
    commands.insert_resource(minigame);

    let water_color = match world.current_location.as_str() {
        "River" => palette::WATER_RIVER,
        "Ocean" => palette::WATER_OCEAN,
        "VolcanicBay" => palette::WATER_VOLCANIC,
        "MistyMarsh" => palette::WATER_MARSH,
        _ => palette::WATER_POND,
    };

    commands.spawn((
        CatchWaterWorld,
        Mesh2d(meshes.add(Rectangle::new(1050.0, 300.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(water_color))),
        Transform::from_xyz(0.0, -40.0, -2.0),
    ));
    commands.spawn((
        CatchWaterWorld,
        Mesh2d(meshes.add(Rectangle::new(1050.0, 70.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(palette::GRASS))),
        Transform::from_xyz(0.0, 130.0, -1.0),
    ));

    let rod_entity = commands
        .spawn((
            CatchRodSprite,
            CatchWaterWorld,
            Transform::from_xyz(-340.0, 0.0, 3.0).with_rotation(Quat::from_rotation_z(-0.18)),
            Visibility::default(),
        ))
        .id();
    commands.entity(rod_entity).with_children(|rod_root| {
        for (mesh, color, offset, scale) in build_rod_meshes() {
            rod_root.spawn((
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
                Transform {
                    translation: offset,
                    scale: Vec3::splat(scale),
                    ..default()
                },
            ));
        }
    });

    let fish_entity = commands
        .spawn((
            CatchFishSprite,
            CatchWaterWorld,
            Transform::from_xyz(-200.0, -20.0, 4.0),
            Visibility::default(),
        ))
        .id();
    commands.entity(fish_entity).with_children(|fish_root| {
        let model = fish_model_for_species(&fish.species);
        if let Some(glow) = rarity_glow_color(&fish.rarity) {
            fish_root.spawn((
                Mesh2d(meshes.add(Circle::new(42.0 * model.scale))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(glow))),
                Transform::from_xyz(0.0, 0.0, -1.0),
            ));
        }
        for (mesh, color, offset, scale) in build_fish_part_meshes(&model) {
            fish_root.spawn((
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
                Transform {
                    translation: offset,
                    scale: Vec3::splat(scale),
                    ..default()
                },
            ));
        }
    });

    commands
        .spawn((
            CatchMinigameRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                min_height: Val::Px(220.0),
                ..default()
            });

            root
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(14.0)),
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(palette::UI_PANEL),
                    BorderColor(palette::UI_BORDER),
                ))
                .with_children(|panel| {
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|header| {
                            header.spawn((
                                Text::new(format!("{} {}", fish.rarity, fish.species)),
                                TextFont {
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(palette::UI_TEXT),
                            ));
                            header
                                .spawn((
                                    Button,
                                    Node {
                                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    BackgroundColor(palette::UI_PRIMARY),
                                    BorderColor(palette::UI_BORDER),
                                    PullNowButton,
                                ))
                                .with_child((
                                    Text::new("Pull Now"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(palette::UI_TEXT),
                                ));
                        });

                    panel.spawn((
                        Text::new("Press Pull Now or SPACE when the marker is in the green zone."),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(palette::UI_TEXT_DIM),
                    ));

                    panel
                        .spawn((
                            CatchBarRoot,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(52.0),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(palette::UI_PANEL_DARK),
                            BorderColor(palette::UI_BORDER),
                        ))
                        .with_children(|bar| {
                            bar.spawn((
                                CatchYellowZone,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(36.0),
                                    top: Val::Px(8.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.82, 0.66, 0.18)),
                            ));
                            bar.spawn((
                                CatchGreenZone,
                                Node {
                                    position_type: PositionType::Absolute,
                                    height: Val::Px(36.0),
                                    top: Val::Px(8.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.18, 0.72, 0.32)),
                            ));
                            bar.spawn((
                                CatchMarker,
                                Node {
                                    position_type: PositionType::Absolute,
                                    width: Val::Px(5.0),
                                    height: Val::Px(52.0),
                                    top: Val::Px(0.0),
                                    ..default()
                                },
                                BackgroundColor(palette::UI_TEXT),
                            ));
                        });

                    panel.spawn((
                        Text::new("Catch Progress: 45%"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(palette::UI_TEXT),
                        CatchProgressLabel,
                    ));
                });
        });
}

#[derive(Component)]
pub struct CatchProgressLabel;

#[derive(Component)]
pub struct PullNowButton;

pub fn catch_minigame_update(
    time: Res<Time>,
    mut minigame: ResMut<CatchMinigame>,
    mut nodes: ParamSet<(
        Query<&mut Node, With<CatchMarker>>,
        Query<&mut Node, With<CatchGreenZone>>,
    )>,
    mut transforms: ParamSet<(
        Query<&mut Transform, With<CatchFishSprite>>,
        Query<&mut Transform, With<CatchRodSprite>>,
    )>,
    mut progress_q: Query<&mut Text, With<CatchProgressLabel>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pull_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PullNowButton>),
    >,
    mut world: ResMut<GameWorld>,
    mut rng: ResMut<crate::GameRng>,
    mut next_state: ResMut<NextState<crate::GameScreen>>,
    mut commands: Commands,
    catch_root: Query<Entity, With<CatchMinigameRoot>>,
    water_world: Query<Entity, With<CatchWaterWorld>>,
) {
    minigame.tick_marker(time.delta_secs());

    for mut node in nodes.p0().iter_mut() {
        node.left = Val::Percent((minigame.marker_position * 100.0) as f32);
    }

    let green_pct = (minigame.green_width * 100.0) as f32;
    for mut node in nodes.p1().iter_mut() {
        node.width = Val::Percent(green_pct);
        node.left = Val::Percent((50.0 - green_pct / 2.0).max(0.0));
    }

    let fish_x = -300.0 + (minigame.marker_position * 460.0) as f32;
    let fish_bob = (minigame.tick as f32 * 0.28).sin() * 8.0;
    for mut transform in transforms.p0().iter_mut() {
        transform.translation.x = fish_x;
        transform.translation.y = -20.0 + fish_bob;
        transform.rotation = Quat::from_rotation_z(
            (minigame.marker_position as f32 - 0.5) * 0.35 + (minigame.tick as f32 * 0.05).sin() * 0.08,
        );
    }

    let pull_tilt = if minigame.pull_frames > 0 { -0.22 } else { -0.18 };
    let rod_tilt = pull_tilt + (minigame.marker_position as f32 - 0.5) * 0.12;
    for mut transform in transforms.p1().iter_mut() {
        transform.rotation = Quat::from_rotation_z(rod_tilt);
        transform.translation.y = 0.0 + minigame.pull_frames as f32 * 0.4;
    }

    for mut text in progress_q.iter_mut() {
        **text = format!("Catch Progress: {}%", minigame.catch_progress);
    }

    let pull_pressed = keyboard.just_pressed(KeyCode::Space);
    let mut button_pull = false;
    for (interaction, mut bg) in pull_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            button_pull = true;
        }
        bg.0 = match *interaction {
            Interaction::Hovered | Interaction::Pressed => palette::UI_PRIMARY_HOVER,
            Interaction::None => palette::UI_PRIMARY,
        };
    }

    if (pull_pressed || button_pull) && !minigame.finished {
        minigame.resolve_pull(&mut rng.0);
        if minigame.pull_count == 3 {
            world.log(format!(
                "Pull reveal: {} {}",
                minigame.fish.rarity, minigame.fish.species
            ));
        }
    }

    if minigame.finished {
        let fish = minigame.fish.clone();
        if minigame.caught {
            world.on_catch_success(fish, &mut rng.0);
        } else {
            world.on_catch_fail(&mut rng.0);
        }
        for entity in catch_root.iter() {
            commands.entity(entity).despawn();
        }
        for entity in water_world.iter() {
            commands.entity(entity).despawn();
        }
        commands.remove_resource::<CatchMinigame>();
        next_state.set(crate::GameScreen::Main);
    }
}

pub fn cleanup_catch_minigame(
    mut commands: Commands,
    catch_root: Query<Entity, With<CatchMinigameRoot>>,
    water_world: Query<Entity, With<CatchWaterWorld>>,
    minigame: Option<Res<CatchMinigame>>,
) {
    if minigame.is_some() {
        for entity in catch_root.iter() {
            commands.entity(entity).despawn();
        }
        for entity in water_world.iter() {
            commands.entity(entity).despawn();
        }
        commands.remove_resource::<CatchMinigame>();
    }
}
