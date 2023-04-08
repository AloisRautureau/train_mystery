mod character;
mod game_state;
pub mod room;
mod train;

use crate::character::CharacterBundle;
use crate::game_state::{GameState, GameplayState, MenuState};
use crate::room::{Room, RoomCharacterStorage};
use crate::train::{Train, ROOMS_COUNT};
use bevy::window::WindowResolution;
use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
    window::WindowResolution,
    render::view::visibility::RenderLayers,
    core_pipeline::clear_color::ClearColorConfig,
};
use std::cmp::min;
use std::fs;

#[derive(Component, Deref, DerefMut, Default)]
pub struct CameraPosition(pub Vec3);

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.),
                resizable: false,
                title: "Train Schizophrenia".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameState {
            gameplay_state: GameplayState::Uninit,
            opened_menu: MenuState::None,
        })
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(animate_sprites)
        .add_system(animate_background)
        .add_system(interpolate_transforms)
        .run()
}

/// Sets up out assets, cameras, etc
fn setup(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // Spawn rooms, characters and train
    let rooms = [(); 7].map(|_| commands.spawn(Room::default()).id());
    game_state.gameplay_state = GameplayState::Room {
        room_id: rooms[0],
        selected_character: 0,
    };
    commands.spawn(Train { rooms });

    for file in fs::read_dir("assets/automata").unwrap() {
        commands.spawn(CharacterBundle::from_json(file.unwrap().path(), &asset_server).unwrap());
    }

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            }, ..default()
        },
        WagonCamera,
        CameraPosition(Camera2dBundle::default().transform.translation),
        RenderLayers::layer(0)
    ));
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                order: 2,
                ..default()
            }, ..default()
        },
        FixedCamera,
        RenderLayers::layer(1)
    ));

    // Load up assets

    // BACKGROUNDS
    let background_texture = asset_server.load("background/background2.png");
    commands.spawn((
        SpriteBundle {
            texture: background_texture,
            sprite: Sprite {
                rect: Some(Rect::new(
                    0f32,
                    0f32,
                    (ROOMS_COUNT as f32) * 1920f32,
                    1080f32,
                )),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1.0, (1080.0) / (1714.0 * 1.5), 1.0))
                .with_translation(Vec3::new(0.0, 1080.0 / 2.5, 0.1)),
            ..default()
        },
        BackgroundAnimation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            speed: 2.0,
            size: 16382.0,
        },
    ));

	let desert_texture = asset_server.load("background/desert2.png");
	commands.spawn((
        SpriteBundle {
            texture: desert_texture,
            sprite: Sprite {
                rect: Some(Rect::new(
                    0f32,
                    0f32,
                    (ROOMS_COUNT as f32) * 1920f32,
                    1080f32,
                )),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1.0, (1080.0 * 1.5) / 1714.0, 1.0))
				.with_translation(Vec3::new(0.0,-1080.0 / 4.0,0.0)),
            ..default()
        },
        BackgroundAnimation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            speed: 10.0,
            size: 16384.0,
        },
    ));

    // TRAINS
    let wagon_texture = asset_server.load("wagon/wagon_ext.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        wagon_texture,
        Vec2::new(945f32, 626f32),
        3,
        1,
        None,
        None,
    ));

    for i in 0..(ROOMS_COUNT - 1) {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                sprite: TextureAtlasSprite::new(i % 3),
                transform: Transform::from_translation(Vec3::new(
                    925f32 * i as f32,
                    0f32,
                    (i % 2) as f32 + 1f32,
                )),
                ..default()
            },
            Animation {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                frames: 3,
            },
        ));
    }

    let locomotive_texture = asset_server.load("locomotive/locomotive.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        locomotive_texture,
        Vec2::new(966f32, 626f32),
        3,
        1,
        None,
        None,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(915f32 * 6f32, 0f32, 1f32)),
            ..default()
        },
        Animation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frames: 3,
        },
    ));

    /*
    let background_image = asset_server.load("background.png");
    commands.spawn(SpriteBundle {
        texture: background_image,
        .. default()
    });
    */
    let audio_train = asset_server.load("audio/train.ogg");
    audio.play_with_settings(
        audio_train,
        PlaybackSettings {
            repeat: true,
            volume: 0.25,
            speed: 1.0,
        },
    );

    let audio_birds = asset_server.load("audio/birds.ogg");
    audio.play_with_settings(
        audio_birds,
        PlaybackSettings {
            repeat: true,
            volume: 1.5,
            speed: 1.0,
        },
    );

    let audio_wind = asset_server.load("audio/wind.ogg");
    audio.play_with_settings(
        audio_wind,
        PlaybackSettings {
            repeat: true,
            volume: 1.5,
            speed: 1.0,
        },
    );

    let audio_wind = asset_server.load("audio/Night-on-the-Docks-Sax.ogg");
    audio.play_with_settings(audio_wind, PlaybackSettings::LOOP);

    let ui_font_handle = asset_server.load("fonts/DejaVuSerif.ttf");
    commands.insert_resource(UiFont(ui_font_handle));
}

/// Deals with player input
fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    train: Query<&Train>,
    rooms: Query<&RoomCharacterStorage>,
    mut camera: Query<&mut CameraPosition, With<WagonCamera>>,
    mut commands: Commands,
    windowq: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let train = train.get_single().unwrap();
    if keys.just_pressed(KeyCode::Left) {
        match game_state.as_mut() {
            GameState {
                opened_menu: MenuState::None,
                gameplay_state: GameplayState::Hub { selected_room },
            } => {
                *selected_room = selected_room.checked_sub(1).unwrap_or(0);
            }
            GameState {
                opened_menu: MenuState::None,
                gameplay_state:
                    GameplayState::Room {
                        selected_character,
                        room_id,
                    },
            } => {
                let characters = rooms.get(*room_id).unwrap();
                let characters_count =
                    characters
                        .0
                        .iter()
                        .fold(0, |acc, c| if c.is_some() { acc + 1 } else { acc });
                if characters_count != 0 {
                    *selected_character = selected_character.checked_sub(1).unwrap_or(0)
                }
            }
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Right) {
        match game_state.as_mut() {
            GameState {
                opened_menu: MenuState::None,
                gameplay_state: GameplayState::Hub { selected_room },
            } => {
                *selected_room = min(*selected_room + 1, ROOMS_COUNT - 1);
            }
            GameState {
                opened_menu: MenuState::None,
                gameplay_state:
                    GameplayState::Room {
                        selected_character,
                        room_id,
                    },
            } => {
                let characters = rooms.get(*room_id).unwrap();
                let characters_count =
                    characters
                        .0
                        .iter()
                        .fold(0, |acc, c| if c.is_some() { acc + 1 } else { acc });
                if characters_count != 0 {
                    *selected_character = (*selected_character + 1) % characters_count;
                }
            }
            _ => (),
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        // Interacts with the closest intractable entity, advances the dialogue or selects a dialogue option
        match game_state.as_mut() {
            GameState {
                gameplay_state: GameplayState::Hub { selected_room },
                opened_menu: MenuState::None,
            } => {
                game_state.gameplay_state = GameplayState::Room {
                    room_id: train.rooms[*selected_room],
                    selected_character: 0,
                }
            }
            GameState {
                gameplay_state: GameplayState::Room { .. },
                opened_menu: MenuState::None,
            } => {
                // Interact with the selected character
            }
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Tab) {
        // Opens or closes journal no matter the game state
        match game_state.opened_menu {
            MenuState::None => game_state.opened_menu = MenuState::Journal,
            MenuState::Journal => game_state.opened_menu = MenuState::None,
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        // Pauses the game
        match game_state.opened_menu {
            MenuState::None => game_state.opened_menu = MenuState::Pause,
            MenuState::Pause => game_state.opened_menu = MenuState::None,
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Back) {
        // Pauses the game
        match game_state.as_mut() {
            GameState {
                gameplay_state: GameplayState::Room { room_id, .. },
                opened_menu: MenuState::None,
            } => {
                let selected_room = train
                    .rooms
                    .iter()
                    .enumerate()
                    .find_map(|(i, r)| if r == room_id { Some(i) } else { None })
                    .unwrap();
                game_state.gameplay_state = GameplayState::Hub { selected_room }
            }
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        match game_state.as_mut() {
            GameState {
                gameplay_state: GameplayState::Room { .. },
                opened_menu: MenuState::None,
            } => {
                show_text(commands, windowq.single(), asset_server, "I'd just like to interject for a moment. What you're refering to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component…");
            }
            _ => (),
        }
    }

    // Deal with camera
    match game_state.as_mut() {
        GameState {
            gameplay_state: GameplayState::Hub { selected_room },
            ..
        } => {
            let mut camera_transform = camera.get_single_mut().unwrap();
            camera_transform.x = 925f32 * *selected_room as f32;
        }
        GameState {
            gameplay_state: GameplayState::Room { .. },
            ..
        } => {}
        _ => (),
    }
}

fn interpolate_transforms(time: Res<Time>, mut query: Query<(&CameraPosition, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation = transform.translation * 0.95 + position.0 * 0.05
    }
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            sprite.index += 1;
            if sprite.index == animation.frames {
                sprite.index = 0;
            }
        }
    }
}

fn animate_background(time: Res<Time>, mut query: Query<(&mut BackgroundAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            // On translate le rectangle de la vue
            let mut rect = sprite.rect.unwrap();
            translate_rectangle(&mut rect, animation.speed, 0.0);
            sprite.rect = Some(rect);
            // Si on est trop loin, on wrap
            if rect.max.x > animation.size {
                sprite.rect = Some(Rect::new(0.0, 0.0, (ROOMS_COUNT as f32) * 1920.0, 1080.0))
            }
        }
    }
}

fn translate_rectangle(rect: &mut Rect, translation_x: f32, translation_y: f32) {
    rect.max.x += translation_x;
    rect.max.y += translation_y;
    rect.min.x += translation_x;
    rect.min.y += translation_y;
}
fn show_text(mut commands: Commands, window: &Window, asset_server: Res<AssetServer>, text: &str) {
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    let font_size = 42.;
    let style = TextStyle {
        font: asset_server.load("fonts/DejaVuSerif.ttf"),
        font_size: font_size,
        color: Color::WHITE,
    };

    let box_size = Vec2::new(window_width*0.9, window_height*0.3);
    let box_pos = Vec2::new(0.0, (window_height as f32)*-0.3);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.1, 0.1, 0.8),
                custom_size: Some(box_size),
                ..default()
            },
            transform: Transform::from_translation(box_pos.extend(0.0)),
            ..default()
        },
        RenderLayers::layer(1)
    )).with_children(|builder| {
        builder.spawn((
            Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        text,
                        style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behaviour: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    size: box_size,
                },
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            },
            RenderLayers::layer(1)
        ));
    });
}

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: usize,
}

#[derive(Component)]
struct WagonCamera;

#[derive(Component)]
pub struct BackgroundAnimation {
    pub timer: Timer,
    pub speed: f32,
    pub size: f32, // Longueur de l'image (pour savoir quand wrap)
}

#[derive(Component)]
struct FixedCamera;

#[derive(Resource)]
struct UiFont(Handle<Font>);
