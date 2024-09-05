#![feature(adt_const_params)]
#![feature(duration_millis_float)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use bevy::{
    input::common_conditions::*,
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

use std::marker::ConstParamTy;

use soundtrack::SoundtrackPlugin;

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
        .add_plugins(SoundtrackPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                duck_move::<{ Direction::Left }>.run_if(input_pressed(KeyCode::KeyA)),
                duck_move::<{ Direction::Right }>.run_if(input_pressed(KeyCode::KeyD)),
                duck_move::<{ Direction::Up }>.run_if(input_pressed(KeyCode::KeyW)),
                duck_move::<{ Direction::Down }>.run_if(input_pressed(KeyCode::KeyS)),
            ),
        )
        .run();
}

#[derive(Component)]
struct Duck;

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
