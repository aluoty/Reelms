use bevy::prelude::*;

use crate::archetype::PlayerArchetype;
use crate::models::palette;
use crate::save::{load_progress, save_progress, SAVE_PATH};
use crate::trader::TraderNpc;
use crate::world::{GameWorld, MAX_STAMINA};

const PANEL_PADDING: f32 = 10.0;
const PANEL_GAP: f32 = 8.0;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct StatusLabel;

#[derive(Component)]
pub struct StatsLabel;

#[derive(Component)]
pub struct WorldLabel;

#[derive(Component)]
pub struct RodLabel;

#[derive(Component)]
pub struct LogLabel;

#[derive(Component)]
pub struct ActionButton(pub Action);

#[derive(Component)]
pub struct PrimaryActionButton;

#[derive(Clone, Copy)]
pub enum Action {
    Cast,
    Shop,
    Sell,
    Trade,
    NewTrade,
    Locations,
    Rest,
    Save,
    CloseOverlay,
    BuyRod,
    BuyBait,
    BuyLuck,
    BuyValue,
    BuyStamina,
    BuySpecialMod,
    ToggleSound,
    SelectCharacter(usize),
    SelectLocation(usize),
    SelectTrader(usize),
    DismissGuide,
}

#[derive(Component)]
pub struct OverlayRoot;

pub fn setup_main_ui(mut commands: Commands, mut world: ResMut<GameWorld>, mut rng: ResMut<crate::GameRng>) {
    let had_save = load_progress(&mut world);
    if had_save {
        world.needs_character_select = false;
        world.show_guide = false;
    }
    world.update_world_cycle(&mut rng.0);
    world.ensure_location_access();

    commands.spawn(Camera2d);

    commands
        .spawn((
            HudRoot,
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
                min_height: Val::Px(180.0),
                ..default()
            });

            root
                .spawn(panel_node(FlexDirection::Column, Some(PANEL_GAP)))
                .with_children(|dock| {
                    dock.spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    })
                    .with_children(|header| {
                        header
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(2.0),
                                ..default()
                            })
                            .with_children(|titles| {
                                spawn_text(titles, "Reelms", 26.0, palette::UI_TEXT);
                                spawn_text(
                                    titles,
                                    "Low Poly Fishing RPG",
                                    13.0,
                                    palette::UI_TEXT_DIM,
                                );
                            });
                        spawn_primary_button(header, "Cast Line", Action::Cast);
                    });

                    dock.spawn((StatusLabel, text_bundle("Ready to fish.", 16.0, false)));

                    dock.spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(PANEL_GAP),
                        ..default()
                    })
                    .with_children(|columns| {
                        columns
                            .spawn(inset_panel())
                            .with_children(|panel| {
                                spawn_text(panel, "Stats", 13.0, palette::UI_TEXT_DIM);
                                panel.spawn((StatsLabel, text_bundle("", 14.0, false)));
                            });
                        columns
                            .spawn(inset_panel())
                            .with_children(|panel| {
                                spawn_text(panel, "World", 13.0, palette::UI_TEXT_DIM);
                                panel.spawn((WorldLabel, text_bundle("", 13.0, false)));
                                panel.spawn((
                                    RodLabel,
                                    text_bundle("", 13.0, false),
                                ));
                            });
                    });

                    dock.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            min_height: Val::Px(88.0),
                            max_height: Val::Px(120.0),
                            overflow: Overflow::clip_y(),
                            ..default()
                        },
                        inset_panel_inner(),
                    ))
                    .with_children(|log_panel| {
                        spawn_text(log_panel, "Catch Log", 13.0, palette::UI_TEXT_DIM);
                        log_panel.spawn((
                            LogLabel,
                            Text::new(""),
                            TextFont {
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(palette::UI_TEXT_DIM),
                            Node {
                                width: Val::Percent(100.0),
                                ..default()
                            },
                        ));
                    });

                    dock.spawn(Node {
                        width: Val::Percent(100.0),
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::FlexStart,
                        column_gap: Val::Px(6.0),
                        row_gap: Val::Px(6.0),
                        ..default()
                    })
                    .with_children(|actions| {
                        for (label, action) in [
                            ("Shop", Action::Shop),
                            ("Sell All", Action::Sell),
                            ("Trade", Action::Trade),
                            ("New Offer", Action::NewTrade),
                            ("Locations", Action::Locations),
                            ("Rest", Action::Rest),
                            ("Save", Action::Save),
                        ] {
                            spawn_action_button(actions, label, action);
                        }
                    });
                });
        });
}

fn panel_node(direction: FlexDirection, row_gap: Option<f32>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: direction,
            padding: UiRect::all(Val::Px(PANEL_PADDING)),
            row_gap: row_gap.map(Val::Px).unwrap_or(Val::Px(0.0)),
            column_gap: Val::Px(PANEL_GAP),
            ..default()
        },
        BackgroundColor(palette::UI_PANEL),
        BorderColor(palette::UI_BORDER),
    )
}

fn inset_panel() -> impl Bundle {
    (
        Node {
            flex_grow: 1.0,
            flex_basis: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            row_gap: Val::Px(4.0),
            ..default()
        },
        inset_panel_inner(),
    )
}

fn inset_panel_inner() -> impl Bundle {
    (
        BackgroundColor(palette::UI_PANEL_DARK),
        BorderColor(palette::UI_BORDER),
    )
}

fn text_bundle(content: &str, size: f32, full_width: bool) -> impl Bundle {
    (
        Text::new(content.to_string()),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(palette::UI_TEXT),
        Node {
            width: if full_width {
                Val::Percent(100.0)
            } else {
                Val::Auto
            },
            ..default()
        },
    )
}

fn spawn_text(parent: &mut ChildSpawnerCommands, content: &str, size: f32, color: Color) {
    parent.spawn((
        Text::new(content.to_string()),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
    ));
}

fn spawn_action_button(parent: &mut ChildSpawnerCommands, label: &str, action: Action) {
    parent
        .spawn((
            Button,
            ActionButton(action),
            Node {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(7.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(palette::UI_ACCENT),
            BorderColor(palette::UI_BORDER),
        ))
        .with_child((
            Text::new(label.to_string()),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(palette::UI_TEXT),
        ));
}

fn spawn_primary_button(parent: &mut ChildSpawnerCommands, label: &str, action: Action) {
    parent
        .spawn((
            Button,
            ActionButton(action),
            PrimaryActionButton,
            Node {
                padding: UiRect::axes(Val::Px(22.0), Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(palette::UI_PRIMARY),
            BorderColor(palette::UI_BORDER),
        ))
        .with_child((
            Text::new(label.to_string()),
            TextFont {
                font_size: 17.0,
                ..default()
            },
            TextColor(palette::UI_TEXT),
        ));
}

fn with_overlay_panel(parent: &mut ChildSpawnerCommands, build: impl FnOnce(&mut ChildSpawnerCommands)) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(22.0)),
                row_gap: Val::Px(10.0),
                min_width: Val::Px(340.0),
                max_width: Val::Px(560.0),
                ..default()
            },
            BackgroundColor(palette::UI_PANEL),
            BorderColor(palette::UI_BORDER),
        ))
        .with_children(build);
}

pub fn refresh_hud(
    world: Res<GameWorld>,
    mut texts: ParamSet<(
        Query<&mut Text, With<StatusLabel>>,
        Query<&mut Text, With<StatsLabel>>,
        Query<&mut Text, With<WorldLabel>>,
        Query<&mut Text, With<RodLabel>>,
        Query<&mut Text, With<LogLabel>>,
    )>,
    mut buttons: Query<
        (&ActionButton, &mut BackgroundColor, Option<&PrimaryActionButton>),
    >,
) {
    if let Ok(mut t) = texts.p0().single_mut() {
        **t = world.status_message.clone();
    }
    if let Ok(mut t) = texts.p1().single_mut() {
        **t = stats_text(&world);
    }
    if let Ok(mut t) = texts.p2().single_mut() {
        **t = world_text(&world);
    }
    if let Ok(mut t) = texts.p3().single_mut() {
        **t = format!(
            "Rod: {}  ·  Catch assist −{:.0}%",
            world.rod_skin_name(),
            world.rod_catch_difficulty_reduction() * 100.0
        );
    }
    if let Ok(mut t) = texts.p4().single_mut() {
        let lines: Vec<_> = world
            .log_lines
            .iter()
            .skip(1)
            .rev()
            .take(6)
            .cloned()
            .collect();
        **t = if lines.is_empty() {
            "No catches yet.".to_string()
        } else {
            lines.into_iter().rev().collect::<Vec<_>>().join("\n")
        };
    }

    for (btn, mut bg, primary) in buttons.iter_mut() {
        let enabled = match btn.0 {
            Action::Cast => !world.casting && world.stamina >= world.stamina_cost_per_cast(),
            Action::Rest => world.rest_manager.can_rest_now(),
            Action::Trade => world
                .trade_offer
                .as_ref()
                .map(|o| o.is_active())
                .unwrap_or(false),
            _ => true,
        };

        bg.0 = if !enabled {
            palette::UI_DISABLED
        } else if primary.is_some() {
            palette::UI_PRIMARY
        } else {
            palette::UI_ACCENT
        };
    }
}

pub fn update_button_hover(
    mut buttons: Query<
        (
            &Interaction,
            &ActionButton,
            &mut BackgroundColor,
            Option<&PrimaryActionButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    world: Res<GameWorld>,
) {
    for (interaction, btn, mut bg, primary) in buttons.iter_mut() {
        let enabled = match btn.0 {
            Action::Cast => !world.casting && world.stamina >= world.stamina_cost_per_cast(),
            Action::Rest => world.rest_manager.can_rest_now(),
            Action::Trade => world
                .trade_offer
                .as_ref()
                .map(|o| o.is_active())
                .unwrap_or(false),
            _ => true,
        };
        if !enabled {
            continue;
        }

        bg.0 = match *interaction {
            Interaction::Hovered | Interaction::Pressed => {
                if primary.is_some() {
                    palette::UI_PRIMARY_HOVER
                } else {
                    palette::UI_ACCENT_HOVER
                }
            }
            Interaction::None => {
                if primary.is_some() {
                    palette::UI_PRIMARY
                } else {
                    palette::UI_ACCENT
                }
            }
        };
    }
}

fn stats_text(world: &GameWorld) -> String {
    let archetype = world.player_character;
    let average = if world.fish_caught > 0 {
        world.total_catch_value as f64 / world.fish_caught as f64
    } else {
        0.0
    };
    let avg_ten = world.average_last_ten();

    format!(
        "Gold {}  ·  Stamina {}/{}\n\
         Rod L{}  ·  Bait L{}  ·  Inv {}\n\
         {}  ·  Mod {}\n\
         Caught {}  ·  Value {}\n\
         Avg {:.1}  ·  Last 10 {:.1}\n\
         Best: {}\n\
         Worst: {}",
        world.gold,
        world.stamina,
        MAX_STAMINA,
        world.rod_level,
        world.bait_level,
        world.inventory.len(),
        archetype.display_name,
        world.special_rod_power,
        world.fish_caught,
        world.total_catch_value,
        average,
        avg_ten,
        fish_summary(&world.most_value_fish),
        fish_summary(&world.least_value_fish),
    )
}

fn fish_summary(fish: &Option<crate::fish::Fish>) -> String {
    match fish {
        Some(f) => format!("{} {} ({}g)", f.rarity, f.species, f.value),
        None => "—".to_string(),
    }
}

fn world_text(world: &GameWorld) -> String {
    let rest_cd = if world.rest_manager.can_rest_now() {
        "ready".to_string()
    } else {
        format!("{}s", world.rest_manager.seconds_remaining())
    };
    format!(
        "Location: {}\n\
         Weather: {}  ·  Moon: {}\n\
         Rest: {}  ·  Luck/Value: {}/{}\n\
         Relations M/F/K: {}/{}/{}\n\
         {}",
        world.current_location,
        world.weather,
        world.moon_phase,
        rest_cd,
        world.luck_buff_casts,
        world.value_buff_casts,
        world.relation_marina,
        world.relation_broker_finn,
        world.relation_tinker_kai,
        world.trade_text(),
    )
}

pub fn handle_main_buttons(
    mut interaction_q: Query<(&Interaction, &ActionButton), Changed<Interaction>>,
    mut world: ResMut<GameWorld>,
    mut next_state: ResMut<NextState<crate::GameScreen>>,
) {
    for (interaction, action) in interaction_q.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action.0 {
            Action::Cast => {
                if world.start_cast() {
                    // cast timer handled in cast_tick system
                }
            }
            Action::Shop => next_state.set(crate::GameScreen::Shop),
            Action::Sell => {
                world.sell_inventory();
            }
            Action::Trade => {
                world.try_trade_offer();
            }
            Action::NewTrade => next_state.set(crate::GameScreen::TradeTraderSelect),
            Action::Locations => next_state.set(crate::GameScreen::LocationSelect),
            Action::Rest => {
                world.rest();
            }
            Action::Save => {
                if save_progress(&world) {
                    world.status_message = "Progress saved.".to_string();
                    world.log(format!("Progress saved to {}", SAVE_PATH));
                } else {
                    world.status_message = "Save failed.".to_string();
                    world.log("Failed to save progress.");
                }
            }
            _ => {}
        }
    }
}

pub fn cast_tick(
    time: Res<Time>,
    mut world: ResMut<GameWorld>,
    mut rng: ResMut<crate::GameRng>,
    mut next_state: ResMut<NextState<crate::GameScreen>>,
) {
    if !world.casting {
        return;
    }
    if let Some(_fish) = world.tick_cast(time.delta_secs(), &mut rng.0) {
        next_state.set(crate::GameScreen::Catching);
    }
}

pub fn setup_character_select(mut commands: Commands) {
    commands
        .spawn((
            OverlayRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
        ))
        .with_children(|overlay| {
            with_overlay_panel(overlay, |panel| {
                spawn_text(panel, "New Save — Character Select", 22.0, palette::UI_TEXT);
                spawn_text(
                    panel,
                    "Pick an angler to begin your run.",
                    14.0,
                    palette::UI_TEXT_DIM,
                );
                for (i, archetype) in PlayerArchetype::ALL.iter().enumerate() {
                    let cast_pct =
                        ((1.0 - archetype.cast_speed_multiplier) * 100.0).round() as i32;
                    let label = format!(
                        "{} — {}g start · Rod L{} · Bait L{} · Cast {}% faster",
                        archetype.display_name,
                        archetype.starting_gold,
                        archetype.starting_rod_level,
                        archetype.starting_bait_level,
                        cast_pct,
                    );
                    spawn_action_button(panel, &label, Action::SelectCharacter(i));
                }
            });
        });
}

pub fn setup_guide(mut commands: Commands) {
    commands
        .spawn((
            OverlayRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|overlay| {
            with_overlay_panel(overlay, |panel| {
                spawn_text(panel, "Quick Guide", 22.0, palette::UI_TEXT);
                panel.spawn(text_bundle(
                    "1. Cast your line, then pull in the green zone.\n\
                     2. Caught fish go to your inventory.\n\
                     3. Sell at the shop or wait for timed trade offers.\n\
                     4. Upgrade rod and bait to improve catches.\n\
                     5. Unlock new locations by catching more fish.\n\
                     6. Build trader relationships for better prices.\n\
                     7. Rest restores stamina on a cooldown.",
                    14.0,
                    false,
                ));
                spawn_action_button(panel, "Got it", Action::DismissGuide);
            });
        });
}

pub fn setup_shop(mut commands: Commands, world: Res<GameWorld>) {
    let discount = ((1.0 - world.shop_discount_multiplier()) * 100.0_f64).round() as i32;
    let featured = TraderNpc::from_display_name(world.best_friend_trader());
    commands
        .spawn((
            OverlayRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.06, 0.08, 0.88)),
        ))
        .with_children(|overlay| {
            spawn_text(
                overlay,
                &format!("Harbor Shop — {}g · {}% discount", world.gold, discount),
                20.0,
                palette::UI_TEXT,
            );
            overlay
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        flex_wrap: FlexWrap::Wrap,
                        align_content: AlignContent::FlexStart,
                        column_gap: Val::Px(8.0),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    inset_panel_inner(),
                ))
                .with_children(|grid| {
                    spawn_action_button(grid, "Rod Upgrade", Action::BuyRod);
                    spawn_action_button(grid, "Bait Upgrade", Action::BuyBait);
                    spawn_action_button(grid, "Luck Buff (+5)", Action::BuyLuck);
                    spawn_action_button(grid, "Value Buff (+5)", Action::BuyValue);
                    spawn_action_button(grid, "Stamina (+30)", Action::BuyStamina);
                    spawn_action_button(grid, "Master Reel Mod", Action::BuySpecialMod);
                    spawn_action_button(
                        grid,
                        &format!("Sound: {}", if world.sounds_enabled { "On" } else { "Off" }),
                        Action::ToggleSound,
                    );
                });
            let mut dialogue_rng = rand::rng();
            spawn_text(
                overlay,
                &format!(
                    "{} says: \"{}\"",
                    featured.display_name,
                    featured.random_dialogue(&mut dialogue_rng)
                ),
                14.0,
                palette::UI_TEXT_DIM,
            );
            spawn_action_button(overlay, "Close Shop", Action::CloseOverlay);
        });
}

pub fn setup_location_select(mut commands: Commands, world: Res<GameWorld>) {
    let locations = [
        ("Pond", "Starter waters", true),
        ("River", "12 catches · 450g", world.is_location_unlocked("River")),
        (
            "Misty Marsh",
            "20 catches · 900g",
            world.is_location_unlocked("MistyMarsh"),
        ),
        ("Ocean", "30 catches · 1500g", world.is_location_unlocked("Ocean")),
        (
            "Volcanic Bay",
            "60 catches · 4200g",
            world.is_location_unlocked("VolcanicBay"),
        ),
    ];
    commands
        .spawn((
            OverlayRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|overlay| {
            with_overlay_panel(overlay, |panel| {
                spawn_text(panel, "Choose Location", 22.0, palette::UI_TEXT);
                for (i, (name, req, unlocked)) in locations.iter().enumerate() {
                    let label = if *unlocked {
                        format!("{name} — {req}")
                    } else {
                        format!("{name} — locked ({req})")
                    };
                    spawn_action_button(panel, &label, Action::SelectLocation(i));
                }
                spawn_action_button(panel, "Cancel", Action::CloseOverlay);
            });
        });
}

pub fn setup_trade_trader_select(mut commands: Commands) {
    commands
        .spawn((
            OverlayRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|overlay| {
            with_overlay_panel(overlay, |panel| {
                spawn_text(panel, "Choose Trader", 22.0, palette::UI_TEXT);
                spawn_text(
                    panel,
                    "Better relationships improve offer rates.",
                    14.0,
                    palette::UI_TEXT_DIM,
                );
                for (i, trader) in TraderNpc::ALL.iter().enumerate() {
                    spawn_action_button(
                        panel,
                        trader.display_name,
                        Action::SelectTrader(i),
                    );
                }
                spawn_action_button(panel, "Cancel", Action::CloseOverlay);
            });
        });
}

pub fn handle_overlay_buttons(
    mut interaction_q: Query<(&Interaction, &ActionButton), Changed<Interaction>>,
    mut world: ResMut<GameWorld>,
    mut next_state: ResMut<NextState<crate::GameScreen>>,
    mut commands: Commands,
    overlay_q: Query<Entity, With<OverlayRoot>>,
    mut rng: ResMut<crate::GameRng>,
) {
    for (interaction, action) in interaction_q.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let close = matches!(
            action.0,
            Action::CloseOverlay | Action::SelectCharacter(_) | Action::SelectLocation(_)
                | Action::SelectTrader(_) | Action::DismissGuide
        );
        match action.0 {
            Action::CloseOverlay => next_state.set(crate::GameScreen::Main),
            Action::BuyRod => {
                world.buy_rod_upgrade();
            }
            Action::BuyBait => {
                world.buy_bait_upgrade();
            }
            Action::BuyLuck => {
                world.buy_luck_buff();
            }
            Action::BuyValue => {
                world.buy_value_buff();
            }
            Action::BuyStamina => {
                world.buy_stamina_potion();
            }
            Action::BuySpecialMod => {
                world.buy_special_rod_mod();
            }
            Action::ToggleSound => {
                world.sounds_enabled = !world.sounds_enabled;
            }
            Action::SelectCharacter(i) => {
                if let Some(archetype) = PlayerArchetype::ALL.get(i) {
                    world.apply_archetype(*archetype);
                    next_state.set(if world.show_guide {
                        crate::GameScreen::Guide
                    } else {
                        crate::GameScreen::Main
                    });
                }
            }
            Action::DismissGuide => {
                world.show_guide = false;
                next_state.set(crate::GameScreen::Main);
            }
            Action::SelectLocation(i) => {
                let names = ["Pond", "River", "MistyMarsh", "Ocean", "VolcanicBay"];
                if let Some(name) = names.get(i) {
                    world.set_location(name);
                }
                next_state.set(crate::GameScreen::Main);
            }
            Action::SelectTrader(i) => {
                if let Some(trader) = TraderNpc::ALL.get(i) {
                    world.generate_trade_offer(trader.display_name, &mut rng.0);
                }
                next_state.set(crate::GameScreen::Main);
            }
            _ => {}
        }
        if close {
            for entity in overlay_q.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn cleanup_overlay(mut commands: Commands, overlay_q: Query<Entity, With<OverlayRoot>>) {
    for entity in overlay_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn check_startup_flow(
    world: Res<GameWorld>,
    mut next_state: ResMut<NextState<crate::GameScreen>>,
    screen: Res<State<crate::GameScreen>>,
) {
    if *screen.get() != crate::GameScreen::Main {
        return;
    }
    if world.needs_character_select {
        next_state.set(crate::GameScreen::CharacterSelect);
    }
}
