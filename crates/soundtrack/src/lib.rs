use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
enum LoadingState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default)]
struct SoundFolder(Handle<LoadedFolder>);

#[derive(Resource)]
struct Soundtrack {
    idx: usize,
    tracks: Vec<Handle<AudioSource>>,
}

pub struct SoundtrackPlugin;

impl Plugin for SoundtrackPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadingState>()
            .add_systems(OnEnter(LoadingState::Setup), load_sound_folder)
            .add_systems(OnEnter(LoadingState::Finished), setup_soundtrack)
            .add_systems(
                Update,
                (
                    check_sounds_loaded.run_if(in_state(LoadingState::Setup)),
                    change_track.run_if(in_state(LoadingState::Finished)),
                ),
            );
    }
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

fn load_sound_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load in all the songs from the Doom Soundtrack at once
    commands.insert_resource(SoundFolder(asset_server.load_folder("ogg")));
}

fn check_sounds_loaded(
    mut next_state: ResMut<NextState<LoadingState>>,
    sound_folder: Res<SoundFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // If the songs have been loaded, setup everything that depends on them
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sound_folder.0) {
            next_state.set(LoadingState::Finished);
        }
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
