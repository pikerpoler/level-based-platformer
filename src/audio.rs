use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

#[derive(Default, Resource)]
struct AudioState {
    pub volume: f32,
}

#[derive(Component)]
struct MusicVolume;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, update_music_volume)
            .add_event::<AudioEvent>()
            .add_systems(Update, sound_events)
            .insert_resource(AudioState { volume: 0.5 })
            .add_systems(Update, adjust_volume);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(AudioBundle {
            source: asset_server.load("audio/background.ogg"),
            settings: PlaybackSettings {
                volume: Volume::default(),
                mode: PlaybackMode::Loop,
                ..Default::default()
            },
            ..default()
        })
        .insert(MusicVolume);
}

fn adjust_volume(keyboard_input: Res<ButtonInput<KeyCode>>, mut audio_state: ResMut<AudioState>) {
    if keyboard_input.just_pressed(KeyCode::Equal) && audio_state.volume < 1.0 {
        audio_state.volume = (audio_state.volume + 0.1).min(1.0);
    } else if keyboard_input.just_pressed(KeyCode::Minus) && audio_state.volume > 0.0 {
        audio_state.volume = (audio_state.volume - 0.1).max(0.0);
    }
}

fn update_music_volume(
    music_controller: Query<&AudioSink, With<MusicVolume>>,
    audio_state: Res<AudioState>,
) {
    if let Ok(sink) = music_controller.get_single() {
        sink.set_volume(audio_state.volume);
    }
}

#[derive(Event)]
pub enum AudioEvent {
    Jump,
    LevelComplete,
}

fn sound_events(
    asset_server: Res<AssetServer>,
    audio_state: Res<AudioState>,
    mut commands: Commands,
    mut events: EventReader<AudioEvent>,
) {
    for event in events.read() {
        match event {
            AudioEvent::Jump => {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/jump.wav"),
                    settings: PlaybackSettings {
                        volume: Volume::new(audio_state.volume),
                        mode: PlaybackMode::Once,
                        ..Default::default()
                    },
                    ..default()
                });
            }
            AudioEvent::LevelComplete => {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/short_yippee.ogg"),
                    settings: PlaybackSettings {
                        volume: Volume::new(audio_state.volume),
                        mode: PlaybackMode::Once,
                        ..Default::default()
                    },
                    ..default()
                });
            }
        }
    }
}
