use bevy::prelude::*;
use bevy::audio::Volume;
use crate::components::BackgroundMusic;
use crate::types::FruitType;

const SFX_POWERUP_WAV: &[u8] = include_bytes!("../assets/sound/SFX_Powerup_01.wav");
const SFX_COIN_WAV: &[u8] = include_bytes!("../assets/sound/coin.wav");
const MUSIC_FASTSONG_MP3: &[u8] = include_bytes!("../assets/music/fastsong.mp3");

#[derive(Resource, Clone)]
pub struct SoundAssets {
    pub coin: Handle<AudioSource>,
    pub powerup: Handle<AudioSource>,
    pub background_music: Handle<AudioSource>,
}

impl SoundAssets {
    /// Return the appropriate sound effect for a collected fruit:
    /// "coin.wav" for all normal fruits, "SFX_Powerup_01.wav" for Hot Pepper.
    pub fn fruit_sound(&self, fruit_type: FruitType) -> Handle<AudioSource> {
        if fruit_type == FruitType::HotPepper {
            self.powerup.clone()
        } else {
            self.coin.clone()
        }
    }
}

pub fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_assets: ResMut<Assets<AudioSource>>,
) {
    let powerup = if std::path::Path::new("assets/sound/SFX_Powerup_01.wav").exists() {
        asset_server.load("sound/SFX_Powerup_01.wav")
    } else {
        audio_assets.add(AudioSource {
            bytes: SFX_POWERUP_WAV.into(),
        })
    };

    let coin = if std::path::Path::new("assets/sound/coin.wav").exists() {
        asset_server.load("sound/coin.wav")
    } else {
        audio_assets.add(AudioSource {
            bytes: SFX_COIN_WAV.into(),
        })
    };

    let background_music = if std::path::Path::new("assets/music/fastsong.mp3").exists() {
        asset_server.load("music/fastsong.mp3")
    } else {
        audio_assets.add(AudioSource {
            bytes: MUSIC_FASTSONG_MP3.into(),
        })
    };

    commands.insert_resource(SoundAssets {
        coin,
        powerup,
        background_music,
    });
}

pub fn start_background_music(
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,
    existing_music: Query<Entity, With<BackgroundMusic>>,
) {
    for entity in &existing_music {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        AudioPlayer::new(sound_assets.background_music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.40)),
        BackgroundMusic,
    ));
}

pub fn stop_background_music(
    mut commands: Commands,
    music_query: Query<Entity, With<BackgroundMusic>>,
) {
    for entity in &music_query {
        commands.entity(entity).despawn();
    }
}
