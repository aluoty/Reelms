use bevy::prelude::*;
use rand::Rng;

use crate::fish::Fish;
use crate::fish_engine::rarity_rank;
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
pub struct CatchWaterScene;

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

pub fn setup_catch_minigame(
    mut commands: Commands,
    world: Res<GameWorld>,
    mut next_state: ResMut<NextState<crate::GameScreen>>,
) {
    let Some(fish) = world.pending_fish.clone() else {
        next_state.set(crate::GameScreen::Main);
        return;
    };

    let minigame = CatchMinigame::new(
        fish,
        world.rod_level,
        world.rod_catch_difficulty_reduction(),
    );
    commands.insert_resource(minigame);

    commands
        .spawn((
            CatchMinigameRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.12, 0.18)),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("Pulling..."),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
            ));

            root.spawn((
                Text::new("Press [Pull Now] or SPACE when marker is in GREEN."),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.85, 0.9)),
                Node {
                    margin: UiRect::horizontal(Val::Px(12.0)),
                    ..default()
                },
            ));

            root.spawn((
                CatchWaterScene,
                Node {
                    width: Val::Percent(95.0),
                    height: Val::Px(220.0),
                    margin: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.36, 0.61, 0.82)),
            ))
            .with_children(|scene| {
                scene.spawn((
                    CatchRodSprite,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(35.0),
                        bottom: Val::Px(34.0),
                        width: Val::Px(200.0),
                        height: Val::Px(48.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.55, 0.35, 0.15)),
                ));
                scene.spawn((
                    CatchFishSprite,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(32.0),
                        height: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.92, 0.94, 0.96)),
                ));
            });

            root.spawn((
                CatchBarRoot,
                Node {
                    width: Val::Percent(90.0),
                    height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(16.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ))
            .with_children(|bar| {
                bar.spawn((
                    CatchYellowZone,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(44.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.88, 0.71, 0.14)),
                ));
                bar.spawn((
                    CatchGreenZone,
                    Node {
                        position_type: PositionType::Absolute,
                        height: Val::Px(44.0),
                        top: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.16, 0.68, 0.26)),
                ));
                bar.spawn((
                    CatchMarker,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(4.0),
                        height: Val::Px(62.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ));
            });

            root.spawn((
                Node {
                    width: Val::Percent(90.0),
                    margin: UiRect::all(Val::Px(8.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|row| {
                row.spawn((
                    Text::new("Catch Progress: 45%"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    CatchProgressLabel,
                ));
                row.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.45, 0.75)),
                    PullNowButton,
                ))
                .with_child((
                    Text::new("Pull Now"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
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
        Query<&mut Node, With<CatchFishSprite>>,
        Query<&mut Node, With<CatchRodSprite>>,
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

    for mut node in nodes.p2().iter_mut() {
        let x = 20.0 + (minigame.marker_position * 60.0) as f32;
        node.left = Val::Percent(x);
        node.bottom = Val::Px(40.0 + (minigame.tick as f32 * 0.28).sin() * 6.0);
    }

    for mut node in nodes.p3().iter_mut() {
        let angle_offset = if minigame.pull_frames > 0 { -12.0 } else { 0.0 };
        let tilt = -11.0 + (minigame.marker_position * 24.0) as f32 + angle_offset;
        node.bottom = Val::Px(34.0 + tilt.abs() * 0.2);
    }

    for mut text in progress_q.iter_mut() {
        **text = format!("Catch Progress: {}%", minigame.catch_progress);
    }

    let pull_pressed = keyboard.just_pressed(KeyCode::Space);
    let mut button_pull = false;
    for (interaction, _) in pull_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            button_pull = true;
        }
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
        commands.remove_resource::<CatchMinigame>();
        next_state.set(crate::GameScreen::Main);
    }
}

pub fn cleanup_catch_minigame(
    mut commands: Commands,
    catch_root: Query<Entity, With<CatchMinigameRoot>>,
    minigame: Option<Res<CatchMinigame>>,
) {
    if minigame.is_some() {
        for entity in catch_root.iter() {
            commands.entity(entity).despawn();
        }
        commands.remove_resource::<CatchMinigame>();
    }
}
