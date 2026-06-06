mod archetype;
mod catch_minigame;
mod fish;
mod fish_engine;
mod rest;
mod save;
mod trader;
mod ui;
mod world;

use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use catch_minigame::{
    catch_minigame_update, cleanup_catch_minigame, setup_catch_minigame,
};
use ui::{
    cast_tick, check_startup_flow, cleanup_overlay, handle_main_buttons, handle_overlay_buttons,
    refresh_hud, setup_character_select, setup_guide, setup_location_select, setup_main_ui,
    setup_shop, setup_trade_trader_select,
};
use world::GameWorld;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum GameScreen {
    #[default]
    Main,
    CharacterSelect,
    Guide,
    Catching,
    Shop,
    LocationSelect,
    TradeTraderSelect,
}

#[derive(Resource)]
struct GameRng(StdRng);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fishing Game".to_string(),
                resolution: (980., 560.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameScreen>()
        .insert_resource(GameWorld::default())
        .insert_resource(GameRng(StdRng::from_os_rng()))
        .add_systems(Startup, setup_main_ui)
        .add_systems(
            Update,
            (
                check_startup_flow,
                refresh_hud,
                handle_main_buttons,
                cast_tick,
            )
                .chain()
                .run_if(in_state(GameScreen::Main)),
        )
        .add_systems(OnEnter(GameScreen::CharacterSelect), setup_character_select)
        .add_systems(OnEnter(GameScreen::Guide), setup_guide)
        .add_systems(OnEnter(GameScreen::Shop), setup_shop)
        .add_systems(OnEnter(GameScreen::LocationSelect), setup_location_select)
        .add_systems(OnEnter(GameScreen::TradeTraderSelect), setup_trade_trader_select)
        .add_systems(
            Update,
            (handle_overlay_buttons,)
                .run_if(
                    in_state(GameScreen::CharacterSelect)
                        .or(in_state(GameScreen::Guide))
                        .or(in_state(GameScreen::Shop))
                        .or(in_state(GameScreen::LocationSelect))
                        .or(in_state(GameScreen::TradeTraderSelect)),
                ),
        )
        .add_systems(
            OnExit(GameScreen::CharacterSelect),
            cleanup_overlay,
        )
        .add_systems(OnExit(GameScreen::Guide), cleanup_overlay)
        .add_systems(OnExit(GameScreen::Shop), cleanup_overlay)
        .add_systems(OnExit(GameScreen::LocationSelect), cleanup_overlay)
        .add_systems(OnExit(GameScreen::TradeTraderSelect), cleanup_overlay)
        .add_systems(OnEnter(GameScreen::Catching), setup_catch_minigame)
        .add_systems(
            Update,
            (catch_minigame_update,).run_if(in_state(GameScreen::Catching)),
        )
        .add_systems(OnExit(GameScreen::Catching), cleanup_catch_minigame)
        .run();
}
