#![feature(adt_const_params)]
#![feature(duration_millis_float)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use bevy::{
    asset::LoadedFolder,
    input::common_conditions::*,
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

use std::marker::ConstParamTy;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(Backends::VULKAN), // Otherwise dx12/vulkan mix on windows causes annoying errors w/ AMD
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Isaac Nudelman: RTF 329C Walking Simulator".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Setup), load_sound_folder)
        .add_systems(OnEnter(AppState::Finished), setup_soundtrack)
        .add_systems(
            Update,
            (
                duck_move::<{ Direction::Left }>.run_if(input_pressed(KeyCode::KeyA)),
                duck_move::<{ Direction::Right }>.run_if(input_pressed(KeyCode::KeyD)),
                duck_move::<{ Direction::Up }>.run_if(input_pressed(KeyCode::KeyW)),
                duck_move::<{ Direction::Down }>.run_if(input_pressed(KeyCode::KeyS)),
                check_sounds_loaded.run_if(in_state(AppState::Setup)),
                change_track.run_if(in_state(AppState::Finished)),
            ),
        )
        .run();
}

#[derive(Component)]
struct Duck;

#[derive(Resource, Default)]
struct SoundFolder(Handle<LoadedFolder>);

fn load_sound_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load in all the songs from the Doom Soundtrack at once
    commands.insert_resource(SoundFolder(asset_server.load_folder("ogg")));
}

fn check_sounds_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    sound_folder: Res<SoundFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // If the songs have been loaded, setup everything that depends on them
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sound_folder.0) {
            next_state.set(AppState::Finished);
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("ducky.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..Default::default()
        })
        .insert(Duck);

    commands.spawn(TextBundle::from_section(
        "Use WASD to move\n",
        TextStyle::default(),
    ));
}

fn setup_soundtrack(
    mut commands: Commands,
    sound_folder: Res<SoundFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
) {
    let mut soundtrack = Soundtrack::new();

    let loaded_folder = loaded_folders.get(&sound_folder.0).unwrap();
    for handle in loaded_folder.handles.iter() {
        let track = handle.clone().typed::<AudioSource>();
        soundtrack.tracks.push(track);
    }

    commands.insert_resource(soundtrack);
}

#[derive(Eq, PartialEq, ConstParamTy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn duck_move<const DIRECTION: Direction>(
    time: Res<Time>,
    mut sprite_positions: Query<&mut Transform, With<Duck>>,
) {
    let mut transform = sprite_positions.single_mut();
    match DIRECTION {
        Direction::Up => transform.translation.y += 2. * time.delta().as_millis_f32(),
        Direction::Down => transform.translation.y -= 2. * time.delta().as_millis_f32(),
        Direction::Left => transform.translation.x -= 2. * time.delta().as_millis_f32(),
        Direction::Right => transform.translation.x += 2. * time.delta().as_millis_f32(),
    };
}

#[derive(Resource)]
struct Soundtrack {
    idx: usize,
    tracks: Vec<Handle<AudioSource>>,
}

impl Soundtrack {
    fn new() -> Self {
        Self {
            idx: 0,
            tracks: vec![],
        }
    }

    fn next(&mut self) -> Handle<AudioSource> {
        let r = self.tracks[self.idx].clone();
        info!("Current track: {:?}", r);

        if self.idx + 1 >= self.tracks.len() {
            self.idx = 0;
        } else {
            self.idx += 1;
        }

        r
    }
}

fn change_track(
    mut commands: Commands,
    mut soundtrack: ResMut<Soundtrack>,
    active_audio: Query<(&AudioSink, Entity)>,
) {
    if active_audio.iter().all(|(audio, _)| audio.empty()) {
        // despawn all old audio players
        active_audio
            .iter()
            .for_each(|(_, entity)| commands.entity(entity).despawn_recursive());

        // spawn new sound player
        commands.spawn(AudioBundle {
            source: soundtrack.next(),
            ..default()
        });
    }
}
