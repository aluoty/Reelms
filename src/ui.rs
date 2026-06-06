use bevy::prelude::*;

use crate::archetype::PlayerArchetype;
use crate::save::{load_progress, save_progress, SAVE_PATH};
use crate::trader::TraderNpc;
use crate::world::{GameWorld, MAX_STAMINA};

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
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.14, 0.18)),
        ))
        .with_children(|root| {
            spawn_text(root, "Fishing Game", 26.0, Color::WHITE);
            root.spawn((
                StatusLabel,
                text_bundle("Ready to fish.", 18.0),
            ));
            root.spawn((StatsLabel, text_bundle("", 15.0)));
            root.spawn((WorldLabel, text_bundle("", 14.0)));
            root.spawn((RodLabel, text_bundle("", 14.0)));

            root.spawn((
                Node {
                    flex_grow: 1.0,
                    width: Val::Percent(100.0),
                    overflow: Overflow::clip(),
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.08, 0.09, 0.11)),
            ))
            .with_children(|log_panel| {
                log_panel.spawn((
                    LogLabel,
                    Text::new("Catch log:\n"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.88, 0.92)),
                    Node {
                        width: Val::Percent(100.0),
                        ..default()
                    },
                ));
            });

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::FlexEnd,
                    column_gap: Val::Px(6.0),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
            ))
            .with_children(|actions| {
                for (label, action) in [
                    ("Shop", Action::Shop),
                    ("Sell Inventory", Action::Sell),
                    ("Trade Offer", Action::Trade),
                    ("New Trade Offer", Action::NewTrade),
                    ("Locations", Action::Locations),
                    ("Rest (+25)", Action::Rest),
                    ("Save", Action::Save),
                    ("Cast Line", Action::Cast),
                ] {
                    spawn_action_button(actions, label, action);
                }
            });
        });
}

fn text_bundle(content: &str, size: f32) -> impl Bundle {
    (
        Text::new(content.to_string()),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.92, 0.95)),
        Node {
            width: Val::Percent(100.0),
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
                padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.22, 0.38, 0.58)),
        ))
        .with_child((
            Text::new(label.to_string()),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
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
    mut buttons: Query<(&ActionButton, &mut BackgroundColor, &mut Node)>,
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
            "Rod Skin: {} | Catch Assist: -{:.0}% difficulty",
            world.rod_skin_name(),
            world.rod_catch_difficulty_reduction() * 100.0
        );
    }
    if let Ok(mut t) = texts.p4().single_mut() {
        **t = world.log_lines.join("\n");
    }

    for (btn, mut bg, mut node) in buttons.iter_mut() {
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
        bg.0 = if enabled {
            Color::srgb(0.22, 0.38, 0.58)
        } else {
            Color::srgb(0.28, 0.28, 0.30)
        };
        node.display = if enabled { Display::Flex } else { Display::Flex };
    }
}

fn stats_text(world: &GameWorld) -> String {
    let archetype = world.player_character;
    if world.fish_caught == 0 {
        return format!(
            "Gold: {} | Stamina: {}/{} | Rod L{} | Bait L{} | Inventory: {} | {} | Special Mod: {}\n\
             Caught: 0 | Total Value: 0 | Avg: 0.00 | Avg Last 10: 0.00\n\
             Least: N/A\nMost: N/A",
            world.gold,
            world.stamina,
            MAX_STAMINA,
            world.rod_level,
            world.bait_level,
            world.inventory.len(),
            archetype.display_name,
            world.special_rod_power,
        );
    }
    let average = world.total_catch_value as f64 / world.fish_caught as f64;
    let avg_ten = world.average_last_ten();
    format!(
        "Gold: {} | Stamina: {}/{} | Rod L{} | Bait L{} | Inventory: {} | {} | Special Mod: {}\n\
         Caught: {} | Total Value: {}\n\
         Avg: {:.2} | Avg Last 10: {:.2}\n\
         Least: {}\nMost: {}",
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
        fish_summary(&world.least_value_fish),
        fish_summary(&world.most_value_fish),
    )
}

fn fish_summary(fish: &Option<crate::fish::Fish>) -> String {
    match fish {
        Some(f) => format!("{} / {} ({})", f.value, f.rarity, f.species),
        None => "N/A".to_string(),
    }
}

fn world_text(world: &GameWorld) -> String {
    let rest_cd = if world.rest_manager.can_rest_now() {
        "0".to_string()
    } else {
        world.rest_manager.seconds_remaining().to_string()
    };
    format!(
        "Location: {} | Weather: {} | Moon: {} | Rest CD: {}s | Buffs(Luck/Value): {}/{} | Rel(M/F/K): {}/{}/{} | {}",
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
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(24.0)),
                        row_gap: Val::Px(12.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.18, 0.24)),
                ))
                .with_children(|panel| {
                    spawn_text(panel, "New Save - Character Select", 24.0, Color::WHITE);
                    for (i, archetype) in PlayerArchetype::ALL.iter().enumerate() {
                        let cast_pct =
                            ((1.0 - archetype.cast_speed_multiplier) * 100.0).round() as i32;
                        let label = format!(
                            "{} | Gold {} | Rod L{} | Bait L{} | Cast {}%",
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
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        max_width: Val::Px(520.0),
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.14, 0.17, 0.22)),
                ))
                .with_children(|panel| {
                    spawn_text(panel, "Quick Guide", 22.0, Color::WHITE);
                    panel.spawn(text_bundle(
                        "1) Cast line -> pull in green zone to catch.\n\
                         2) Fish go to inventory.\n\
                         3) Sell inventory or wait for trade offers.\n\
                         4) Upgrade rod/bait in Shop.\n\
                         5) Unlock locations via catch milestones.\n\
                         6) Build trader relationships for better deals.\n\
                         7) Rest restores stamina with a cooldown.",
                        15.0,
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
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.07, 0.10, 0.92)),
        ))
        .with_children(|overlay| {
            spawn_text(
                overlay,
                &format!(
                    "Harbor Shop | Gold: {} | Discount: {}%",
                    world.gold, discount
                ),
                20.0,
                Color::WHITE,
            );
            overlay.spawn((
                Node {
                    flex_grow: 1.0,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(8.0),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
            ))
            .with_children(|grid| {
                spawn_action_button(grid, "Rod Upgrade", Action::BuyRod);
                spawn_action_button(grid, "Bait Upgrade", Action::BuyBait);
                spawn_action_button(grid, "Luck Buff (+5 casts)", Action::BuyLuck);
                spawn_action_button(grid, "Value Buff (+5 casts)", Action::BuyValue);
                spawn_action_button(grid, "Stamina Potion (+30)", Action::BuyStamina);
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
                    "Featured: {} — \"{}\"",
                    featured.display_name,
                    featured.random_dialogue(&mut dialogue_rng)
                ),
                16.0,
                Color::srgb(0.8, 0.85, 0.9),
            );
            spawn_action_button(overlay, "Close Shop", Action::CloseOverlay);
        });
}

pub fn setup_location_select(mut commands: Commands, world: Res<GameWorld>) {
    let locations = [
        ("Pond", true),
        ("River", world.is_location_unlocked("River")),
        ("Ocean", world.is_location_unlocked("Ocean")),
        ("VolcanicBay", world.is_location_unlocked("VolcanicBay")),
    ];
    commands
        .spawn((
            OverlayRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.14, 0.17, 0.22)),
                ))
                .with_children(|panel| {
                    spawn_text(panel, "Choose Location", 22.0, Color::WHITE);
                    for (i, (name, unlocked)) in locations.iter().enumerate() {
                        let suffix = if *unlocked {
                            String::new()
                        } else {
                            " [Locked]".to_string()
                        };
                        spawn_action_button(panel, &format!("{}{}", name, suffix), Action::SelectLocation(i));
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
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.14, 0.17, 0.22)),
                ))
                .with_children(|panel| {
                    spawn_text(panel, "Choose Trader", 22.0, Color::WHITE);
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
                let names = ["Pond", "River", "Ocean", "VolcanicBay"];
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
