use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

/// Flat, saturated palette for a stylized low-poly look.
pub mod palette {
    use bevy::prelude::Color;

    pub const SKY_DAWN: Color = Color::srgb(0.55, 0.72, 0.88);
    pub const SKY_CLEAR: Color = Color::srgb(0.42, 0.68, 0.92);
    pub const SKY_STORM: Color = Color::srgb(0.28, 0.34, 0.42);
    pub const SKY_NIGHT: Color = Color::srgb(0.12, 0.14, 0.28);
    pub const SKY_VOLCANIC: Color = Color::srgb(0.38, 0.22, 0.18);

    pub const WATER_POND: Color = Color::srgb(0.28, 0.58, 0.72);
    pub const WATER_RIVER: Color = Color::srgb(0.22, 0.52, 0.68);
    pub const WATER_OCEAN: Color = Color::srgb(0.14, 0.38, 0.62);
    pub const WATER_VOLCANIC: Color = Color::srgb(0.62, 0.28, 0.18);
    pub const WATER_MARSH: Color = Color::srgb(0.18, 0.48, 0.38);

    pub const GRASS: Color = Color::srgb(0.38, 0.62, 0.32);
    pub const GRASS_DARK: Color = Color::srgb(0.28, 0.48, 0.24);
    pub const SAND: Color = Color::srgb(0.82, 0.74, 0.52);
    pub const ROCK: Color = Color::srgb(0.48, 0.46, 0.44);
    pub const ROCK_DARK: Color = Color::srgb(0.32, 0.30, 0.28);
    pub const WOOD: Color = Color::srgb(0.52, 0.36, 0.22);
    pub const WOOD_LIGHT: Color = Color::srgb(0.68, 0.50, 0.30);
    pub const PINE: Color = Color::srgb(0.18, 0.38, 0.28);
    pub const PINE_LIGHT: Color = Color::srgb(0.28, 0.52, 0.36);
    pub const LAVA: Color = Color::srgb(0.95, 0.42, 0.12);
    pub const MOON: Color = Color::srgb(0.92, 0.90, 0.72);
    pub const MOON_BLUE: Color = Color::srgb(0.62, 0.78, 0.95);
    pub const MOON_BLOOD: Color = Color::srgb(0.88, 0.28, 0.22);

    pub const UI_PANEL: Color = Color::srgba(0.10, 0.12, 0.16, 0.82);
    pub const UI_PANEL_DARK: Color = Color::srgba(0.06, 0.08, 0.10, 0.88);
    pub const UI_ACCENT: Color = Color::srgb(0.32, 0.58, 0.78);
    pub const UI_ACCENT_HOVER: Color = Color::srgb(0.40, 0.68, 0.88);
    pub const UI_TEXT: Color = Color::srgb(0.94, 0.96, 0.98);
    pub const UI_TEXT_DIM: Color = Color::srgb(0.72, 0.76, 0.82);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FishBodyStyle {
    Streamlined,
    Round,
    Flat,
    Eel,
    Spiny,
    Ray,
    Lava,
}

#[derive(Clone, Debug)]
pub struct FishModelDef {
    pub body_style: FishBodyStyle,
    pub body_color: Color,
    pub accent_color: Color,
    pub fin_color: Color,
    pub scale: f32,
}

#[derive(Clone, Debug)]
pub struct SceneryPiece {
    pub mesh: Mesh,
    pub color: Color,
    pub translation: Vec3,
    pub rotation: f32,
    pub scale: Vec3,
}

#[derive(Clone, Debug)]
pub struct LocationTheme {
    pub sky: Color,
    pub water: Color,
    pub ground: Color,
    pub accent: Color,
    pub pieces: Vec<SceneryPiece>,
}

pub fn fish_model_for_species(species: &str) -> FishModelDef {
    match species {
        "Carp" | "Bluegill" | "Perch" => FishModelDef {
            body_style: FishBodyStyle::Round,
            body_color: Color::srgb(0.78, 0.58, 0.28),
            accent_color: Color::srgb(0.92, 0.72, 0.38),
            fin_color: Color::srgb(0.62, 0.42, 0.18),
            scale: 1.0,
        },
        "Salmon" | "Trout" | "Walleye" => FishModelDef {
            body_style: FishBodyStyle::Streamlined,
            body_color: Color::srgb(0.82, 0.42, 0.32),
            accent_color: Color::srgb(0.95, 0.58, 0.42),
            fin_color: Color::srgb(0.68, 0.32, 0.24),
            scale: 1.05,
        },
        "Catfish" | "Sturgeon" => FishModelDef {
            body_style: FishBodyStyle::Flat,
            body_color: Color::srgb(0.48, 0.44, 0.40),
            accent_color: Color::srgb(0.58, 0.52, 0.46),
            fin_color: Color::srgb(0.38, 0.34, 0.30),
            scale: 1.2,
        },
        "Pike" | "Barracuda" => FishModelDef {
            body_style: FishBodyStyle::Spiny,
            body_color: Color::srgb(0.32, 0.52, 0.38),
            accent_color: Color::srgb(0.48, 0.68, 0.52),
            fin_color: Color::srgb(0.22, 0.38, 0.28),
            scale: 1.15,
        },
        "Bass" | "Snapper" => FishModelDef {
            body_style: FishBodyStyle::Streamlined,
            body_color: Color::srgb(0.28, 0.48, 0.32),
            accent_color: Color::srgb(0.42, 0.62, 0.44),
            fin_color: Color::srgb(0.18, 0.32, 0.22),
            scale: 1.0,
        },
        "GoldenKoi" | "PhoenixKoi" => FishModelDef {
            body_style: FishBodyStyle::Round,
            body_color: Color::srgb(0.98, 0.72, 0.18),
            accent_color: Color::srgb(1.0, 0.88, 0.42),
            fin_color: Color::srgb(0.92, 0.48, 0.12),
            scale: 1.25,
        },
        "SilverEel" => FishModelDef {
            body_style: FishBodyStyle::Eel,
            body_color: Color::srgb(0.78, 0.82, 0.88),
            accent_color: Color::srgb(0.92, 0.94, 0.98),
            fin_color: Color::srgb(0.58, 0.62, 0.68),
            scale: 1.1,
        },
        "Tuna" | "Mackerel" | "BluefinTitan" => FishModelDef {
            body_style: FishBodyStyle::Streamlined,
            body_color: Color::srgb(0.22, 0.38, 0.58),
            accent_color: Color::srgb(0.38, 0.55, 0.78),
            fin_color: Color::srgb(0.14, 0.28, 0.48),
            scale: 1.3,
        },
        "Swordfish" | "Marlin" => FishModelDef {
            body_style: FishBodyStyle::Spiny,
            body_color: Color::srgb(0.32, 0.42, 0.62),
            accent_color: Color::srgb(0.52, 0.62, 0.82),
            fin_color: Color::srgb(0.18, 0.28, 0.48),
            scale: 1.45,
        },
        "Glowfin" | "Anglerfish" => FishModelDef {
            body_style: FishBodyStyle::Spiny,
            body_color: Color::srgb(0.18, 0.72, 0.82),
            accent_color: Color::srgb(0.42, 0.92, 0.98),
            fin_color: Color::srgb(0.62, 0.98, 0.88),
            scale: 1.2,
        },
        "Flounder" => FishModelDef {
            body_style: FishBodyStyle::Flat,
            body_color: Color::srgb(0.62, 0.58, 0.42),
            accent_color: Color::srgb(0.78, 0.72, 0.52),
            fin_color: Color::srgb(0.48, 0.44, 0.32),
            scale: 1.0,
        },
        "LavaSnapper" | "AshGrouper" => FishModelDef {
            body_style: FishBodyStyle::Lava,
            body_color: Color::srgb(0.72, 0.32, 0.18),
            accent_color: Color::srgb(0.95, 0.52, 0.18),
            fin_color: Color::srgb(0.48, 0.18, 0.12),
            scale: 1.1,
        },
        "MagmaRay" => FishModelDef {
            body_style: FishBodyStyle::Ray,
            body_color: Color::srgb(0.88, 0.38, 0.12),
            accent_color: Color::srgb(0.98, 0.62, 0.22),
            fin_color: Color::srgb(0.62, 0.22, 0.08),
            scale: 1.35,
        },
        "ReedMinnow" | "MistyDarter" => FishModelDef {
            body_style: FishBodyStyle::Streamlined,
            body_color: Color::srgb(0.42, 0.62, 0.72),
            accent_color: Color::srgb(0.58, 0.78, 0.88),
            fin_color: Color::srgb(0.28, 0.48, 0.58),
            scale: 0.75,
        },
        "CrystalCarp" | "GlowPike" => FishModelDef {
            body_style: FishBodyStyle::Round,
            body_color: Color::srgb(0.52, 0.82, 0.92),
            accent_color: Color::srgb(0.72, 0.95, 0.98),
            fin_color: Color::srgb(0.38, 0.62, 0.78),
            scale: 1.15,
        },
        _ => FishModelDef {
            body_style: FishBodyStyle::Streamlined,
            body_color: Color::srgb(0.72, 0.78, 0.82),
            accent_color: Color::srgb(0.88, 0.92, 0.95),
            fin_color: Color::srgb(0.52, 0.58, 0.62),
            scale: 1.0,
        },
    }
}

pub fn rarity_glow_color(rarity: &str) -> Option<Color> {
    match rarity {
        "Uncommon" => Some(Color::srgba(0.4, 0.8, 0.4, 0.35)),
        "Rare" => Some(Color::srgba(0.3, 0.5, 0.95, 0.4)),
        "Epic" => Some(Color::srgba(0.7, 0.3, 0.95, 0.45)),
        "Legendary" => Some(Color::srgba(0.95, 0.75, 0.2, 0.5)),
        "Mythic" => Some(Color::srgba(0.95, 0.35, 0.55, 0.55)),
        _ => None,
    }
}

fn tri_mesh(points: &[[f32; 2]]) -> Mesh {
    let positions: Vec<[f32; 3]> = points.iter().map(|p| [p[0], p[1], 0.0]).collect();
    let indices: Vec<u32> = (1..(points.len() as u32) - 1)
        .flat_map(|i| [0, i, i + 1])
        .collect();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn quad_mesh(w: f32, h: f32) -> Mesh {
    tri_mesh(&[
        [-w / 2.0, -h / 2.0],
        [w / 2.0, -h / 2.0],
        [w / 2.0, h / 2.0],
        [-w / 2.0, h / 2.0],
    ])
}

pub fn build_fish_part_meshes(model: &FishModelDef) -> Vec<(Mesh, Color, Vec3, f32)> {
    let s = model.scale;
    let mut parts = Vec::new();

    match model.body_style {
        FishBodyStyle::Streamlined => {
            parts.push((
                tri_mesh(&[[-28.0, 0.0], [22.0, -10.0], [22.0, 10.0]]),
                model.body_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[-38.0, 0.0], [-28.0, -8.0], [-28.0, 8.0]]),
                model.accent_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[0.0, 10.0], [8.0, 18.0], [16.0, 10.0]]),
                model.fin_color,
                Vec3::ZERO,
                s,
            ));
        }
        FishBodyStyle::Round => {
            parts.push((
                tri_mesh(&[[-18.0, 0.0], [0.0, -14.0], [18.0, 0.0]]),
                model.body_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[-18.0, 0.0], [18.0, 0.0], [0.0, 14.0]]),
                model.accent_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[-22.0, 0.0], [-30.0, -6.0], [-30.0, 6.0]]),
                model.fin_color,
                Vec3::ZERO,
                s,
            ));
        }
        FishBodyStyle::Flat => {
            parts.push((
                quad_mesh(44.0, 22.0),
                model.body_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[-24.0, 0.0], [-34.0, -10.0], [-34.0, 10.0]]),
                model.fin_color,
                Vec3::ZERO,
                s,
            ));
        }
        FishBodyStyle::Eel => {
            parts.push((
                tri_mesh(&[[-40.0, 0.0], [0.0, -6.0], [0.0, 6.0]]),
                model.body_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[0.0, 0.0], [36.0, -5.0], [36.0, 5.0]]),
                model.accent_color,
                Vec3::ZERO,
                s,
            ));
        }
        FishBodyStyle::Spiny => {
            parts.push((
                tri_mesh(&[[-32.0, 0.0], [26.0, -8.0], [26.0, 8.0]]),
                model.body_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[10.0, 8.0], [38.0, 0.0], [10.0, -8.0]]),
                model.accent_color,
                Vec3::new(8.0, 0.0, 0.0),
                s,
            ));
            parts.push((
                tri_mesh(&[[-4.0, 12.0], [6.0, 22.0], [16.0, 12.0]]),
                model.fin_color,
                Vec3::ZERO,
                s,
            ));
        }
        FishBodyStyle::Ray => {
            parts.push((
                tri_mesh(&[[-20.0, 0.0], [0.0, -28.0], [20.0, 0.0]]),
                model.body_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[-20.0, 0.0], [20.0, 0.0], [0.0, 28.0]]),
                model.accent_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[0.0, 0.0], [24.0, -4.0], [24.0, 4.0]]),
                model.fin_color,
                Vec3::ZERO,
                s,
            ));
        }
        FishBodyStyle::Lava => {
            parts.push((
                tri_mesh(&[[-24.0, 0.0], [20.0, -12.0], [20.0, 12.0]]),
                model.body_color,
                Vec3::ZERO,
                s,
            ));
            parts.push((
                tri_mesh(&[[-8.0, 8.0], [4.0, 16.0], [16.0, 8.0]]),
                palette::LAVA,
                Vec3::ZERO,
                s * 0.8,
            ));
            parts.push((
                tri_mesh(&[[8.0, -10.0], [18.0, -4.0], [8.0, 2.0]]),
                palette::LAVA,
                Vec3::ZERO,
                s * 0.7,
            ));
        }
    }

    parts
}

pub fn build_rod_meshes() -> Vec<(Mesh, Color, Vec3, f32)> {
    vec![
        (
            quad_mesh(160.0, 6.0),
            palette::WOOD,
            Vec3::new(-20.0, 0.0, 0.0),
            1.0,
        ),
        (
            tri_mesh(&[[60.0, 0.0], [72.0, 4.0], [72.0, -4.0]]),
            palette::WOOD_LIGHT,
            Vec3::ZERO,
            1.0,
        ),
        (
            quad_mesh(3.0, 28.0),
            Color::srgb(0.82, 0.84, 0.86),
            Vec3::new(74.0, -14.0, 0.0),
            1.0,
        ),
    ]
}

fn hill(x: f32, y: f32, w: f32, h: f32, color: Color) -> SceneryPiece {
    SceneryPiece {
        mesh: tri_mesh(&[[x, y], [x + w, y], [x + w / 2.0, y + h]]),
        color,
        translation: Vec3::ZERO,
        rotation: 0.0,
        scale: Vec3::ONE,
    }
}

fn pine_tree(x: f32, y: f32, scale: f32) -> Vec<SceneryPiece> {
    vec![
        SceneryPiece {
            mesh: quad_mesh(8.0 * scale, 24.0 * scale),
            color: palette::WOOD,
            translation: Vec3::new(x, y + 12.0 * scale, 0.0),
            rotation: 0.0,
            scale: Vec3::ONE,
        },
        SceneryPiece {
            mesh: tri_mesh(&[
                [x - 18.0 * scale, y + 18.0 * scale],
                [x + 18.0 * scale, y + 18.0 * scale],
                [x, y + 48.0 * scale],
            ]),
            color: palette::PINE,
            translation: Vec3::ZERO,
            rotation: 0.0,
            scale: Vec3::ONE,
        },
        SceneryPiece {
            mesh: tri_mesh(&[
                [x - 14.0 * scale, y + 32.0 * scale],
                [x + 14.0 * scale, y + 32.0 * scale],
                [x, y + 56.0 * scale],
            ]),
            color: palette::PINE_LIGHT,
            translation: Vec3::ZERO,
            rotation: 0.0,
            scale: Vec3::ONE,
        },
    ]
}

fn rock_cluster(x: f32, y: f32) -> Vec<SceneryPiece> {
    vec![
        SceneryPiece {
            mesh: tri_mesh(&[[x, y], [x + 22.0, y], [x + 10.0, y + 16.0]]),
            color: palette::ROCK,
            translation: Vec3::ZERO,
            rotation: 0.0,
            scale: Vec3::ONE,
        },
        SceneryPiece {
            mesh: tri_mesh(&[[x + 14.0, y], [x + 34.0, y + 2.0], [x + 22.0, y + 14.0]]),
            color: palette::ROCK_DARK,
            translation: Vec3::ZERO,
            rotation: 0.0,
            scale: Vec3::ONE,
        },
    ]
}

pub fn location_theme(location: &str, weather: &str, moon: &str) -> LocationTheme {
    use palette::*;

    let sky = match (location, weather, moon) {
        (_, _, "BloodMoon") => SKY_NIGHT,
        (_, "Storm", _) => SKY_STORM,
        ("VolcanicBay", _, _) => SKY_VOLCANIC,
        ("MistyMarsh", _, _) if weather == "Rain" => Color::srgb(0.32, 0.38, 0.42),
        _ => SKY_CLEAR,
    };

    let water = match location {
        "River" => WATER_RIVER,
        "Ocean" => WATER_OCEAN,
        "VolcanicBay" => WATER_VOLCANIC,
        "MistyMarsh" => WATER_MARSH,
        _ => WATER_POND,
    };

    let ground = match location {
        "Ocean" | "VolcanicBay" => SAND,
        "MistyMarsh" => GRASS_DARK,
        _ => GRASS,
    };

    let accent = match location {
        "River" => PINE,
        "Ocean" => Color::srgb(0.92, 0.94, 0.98),
        "VolcanicBay" => LAVA,
        "MistyMarsh" => Color::srgb(0.48, 0.62, 0.52),
        _ => WOOD,
    };

    let mut pieces = vec![
        SceneryPiece {
            mesh: quad_mesh(980.0, 180.0),
            color: water,
            translation: Vec3::new(0.0, -140.0, -5.0),
            rotation: 0.0,
            scale: Vec3::ONE,
        },
        SceneryPiece {
            mesh: quad_mesh(980.0, 120.0),
            color: ground,
            translation: Vec3::new(0.0, -50.0, -4.0),
            rotation: 0.0,
            scale: Vec3::ONE,
        },
    ];

    match location {
        "Pond" => {
            pieces.push(hill(-420.0, -20.0, 280.0, 90.0, GRASS_DARK));
            pieces.push(hill(180.0, -30.0, 320.0, 110.0, GRASS));
            pieces.extend(pine_tree(-280.0, -10.0, 0.9));
            pieces.extend(pine_tree(320.0, -18.0, 1.1));
            pieces.push(SceneryPiece {
                mesh: quad_mesh(60.0, 8.0),
                color: WOOD,
                translation: Vec3::new(-120.0, -42.0, 1.0),
                rotation: -0.08,
                scale: Vec3::ONE,
            });
            pieces.extend(rock_cluster(-60.0, -48.0));
        }
        "River" => {
            pieces.push(hill(-380.0, -10.0, 260.0, 80.0, GRASS_DARK));
            pieces.extend(pine_tree(-200.0, -8.0, 1.0));
            pieces.extend(pine_tree(80.0, -12.0, 0.85));
            pieces.extend(pine_tree(260.0, -6.0, 1.15));
            for i in 0..4 {
                pieces.push(SceneryPiece {
                    mesh: tri_mesh(&[
                        [-30.0, 0.0],
                        [30.0, 0.0],
                        [0.0, 8.0],
                    ]),
                    color: Color::srgba(0.32, 0.62, 0.78, 0.55),
                    translation: Vec3::new(-300.0 + i as f32 * 180.0, -155.0, 2.0),
                    rotation: 0.0,
                    scale: Vec3::ONE,
                });
            }
        }
        "Ocean" => {
            pieces.push(hill(-350.0, -25.0, 200.0, 60.0, SAND));
            pieces.push(hill(250.0, -30.0, 280.0, 70.0, SAND));
            pieces.push(SceneryPiece {
                mesh: tri_mesh(&[[0.0, 0.0], [40.0, 0.0], [20.0, 55.0]]),
                color: WOOD,
                translation: Vec3::new(300.0, -35.0, 1.0),
                rotation: 0.0,
                scale: Vec3::ONE,
            });
            for i in 0..5 {
                pieces.push(SceneryPiece {
                    mesh: tri_mesh(&[[-20.0, 0.0], [20.0, 0.0], [0.0, 12.0]]),
                    color: Color::srgba(0.22, 0.48, 0.72, 0.6),
                    translation: Vec3::new(-380.0 + i as f32 * 160.0, -168.0, 2.0),
                    rotation: 0.0,
                    scale: Vec3::ONE,
                });
            }
        }
        "VolcanicBay" => {
            pieces.push(SceneryPiece {
                mesh: tri_mesh(&[[-80.0, 0.0], [80.0, 0.0], [0.0, 140.0]]),
                color: ROCK_DARK,
                translation: Vec3::new(220.0, 20.0, -2.0),
                rotation: 0.0,
                scale: Vec3::ONE,
            });
            pieces.push(SceneryPiece {
                mesh: tri_mesh(&[[-40.0, 0.0], [40.0, 0.0], [0.0, 50.0]]),
                color: LAVA,
                translation: Vec3::new(220.0, 95.0, -1.0),
                rotation: 0.0,
                scale: Vec3::ONE,
            });
            pieces.extend(rock_cluster(-200.0, -45.0));
            pieces.extend(rock_cluster(40.0, -50.0));
            pieces.push(SceneryPiece {
                mesh: quad_mesh(980.0, 40.0),
                color: Color::srgba(0.95, 0.42, 0.12, 0.25),
                translation: Vec3::new(0.0, -175.0, 3.0),
                rotation: 0.0,
                scale: Vec3::ONE,
            });
        }
        "MistyMarsh" => {
            pieces.push(hill(-400.0, -15.0, 300.0, 70.0, GRASS_DARK));
            pieces.push(hill(120.0, -20.0, 350.0, 85.0, GRASS));
            for i in 0..6 {
                pieces.push(SceneryPiece {
                    mesh: quad_mesh(6.0, 40.0 + (i % 3) as f32 * 8.0),
                    color: Color::srgb(0.42, 0.58, 0.38),
                    translation: Vec3::new(-320.0 + i as f32 * 110.0, -30.0, 1.0),
                    rotation: 0.0,
                    scale: Vec3::ONE,
                });
            }
            pieces.push(SceneryPiece {
                mesh: tri_mesh(&[[-25.0, 0.0], [25.0, 0.0], [0.0, 18.0]]),
                color: Color::srgba(0.72, 0.88, 0.92, 0.35),
                translation: Vec3::new(-80.0, -120.0, 2.0),
                rotation: 0.0,
                scale: Vec3::ONE,
            });
        }
        _ => {}
    }

    if moon == "BlueMoon" {
        pieces.push(SceneryPiece {
            mesh: quad_mesh(48.0, 48.0),
            color: MOON_BLUE,
            translation: Vec3::new(360.0, 200.0, -8.0),
            rotation: 0.0,
            scale: Vec3::ONE,
        });
    } else if moon == "BloodMoon" {
        pieces.push(SceneryPiece {
            mesh: quad_mesh(52.0, 52.0),
            color: MOON_BLOOD,
            translation: Vec3::new(340.0, 190.0, -8.0),
            rotation: 0.0,
            scale: Vec3::ONE,
        });
    } else if weather == "Clear" {
        pieces.push(SceneryPiece {
            mesh: quad_mesh(36.0, 36.0),
            color: MOON,
            translation: Vec3::new(380.0, 210.0, -8.0),
            rotation: 0.0,
            scale: Vec3::ONE,
        });
    }

    LocationTheme {
        sky,
        water,
        ground,
        accent,
        pieces,
    }
}
