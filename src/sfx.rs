use crate::SceneMusic;
use agb::eprintln;
use agb::sound::mixer::{ChannelId, Mixer, SoundChannel, SoundData};

pub fn play_sfx(mixer: &mut Mixer, sfx_enabled: bool, track: SoundData) {
    if sfx_enabled {
        let mut sfx = SoundChannel::new(track);
        sfx.stereo();
        mixer.play_sound(sfx);
    }
}

pub fn start_track(
    mixer: &mut Mixer,
    track: SoundData,
    kind: SceneMusic,
) -> (SceneMusic, ChannelId) {
    let mut channel = SoundChannel::new_high_priority(track);
    channel.should_loop().stereo();
    match mixer.play_sound(channel) {
        None => panic!("Unable to start bgm {:?}", kind),
        Some(id) => (kind, id),
    }
}

pub fn stop_bgm(mixer: &mut Mixer, current: (SceneMusic, ChannelId)) {
    if let Some(channel) = mixer.channel(&current.1) {
        channel.stop();
    } else {
        eprintln!("bgm channel missing {:?}", current.0);
    }
}

pub fn update_bgm(
    mixer: &mut Mixer,
    track: SoundData,
    kind: SceneMusic,
    current: Option<(SceneMusic, ChannelId)>,
) -> (SceneMusic, ChannelId) {
    if let Some(current) = current {
        if current.0 != kind {
            stop_bgm(mixer, current);
            start_track(mixer, track, kind)
        } else {
            current
        }
    } else {
        start_track(mixer, track, kind)
    }
}

pub fn init_bgm(
    mixer: &mut Mixer,
    track: SoundData,
    kind: SceneMusic,
    bgm: Option<(SceneMusic, ChannelId)>,
    music_enabled: bool,
) -> Option<(SceneMusic, ChannelId)> {
    if music_enabled {
        Some(update_bgm(mixer, track, kind, bgm))
    } else {
        if let Some(bgm) = bgm {
            stop_bgm(mixer, bgm);
        }
        None
    }
}
