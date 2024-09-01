#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use bevy::{
    prelude::*,
    render::{settings::{Backends, WgpuSettings}, RenderPlugin}
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::VULKAN), // Otherwise dx12/vulkan mix on windows causes annoying errors w/ AMD
                    ..default()
                }
                    .into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ducky.png"),
        ..Default::default()
    });
}
