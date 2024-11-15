#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};
// use bevy_infinite_grid::*;
// use duck_move::DuckMovePlugin;
use fps_player::FPSPlayerPlugin;
use soundtrack::SoundtrackPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        #[cfg(not(target_os = "macos"))]
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
        // .add_plugins(SoundtrackPlugin)
        // .add_plugins(DuckMovePlugin)
        .add_plugins(FPSPlayerPlugin)
        // .add_plugins(InfiniteGridPlugin)
        // .add_systems(Startup, setup_infinite_grid)
        .run();
}

// fn setup_infinite_grid(mut commands: Commands) {
//     commands.spawn(InfiniteGridBundle::default());
// }
