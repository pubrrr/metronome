use std::time::Duration;

use bevy::prelude::{
    default, App, AssetServer, AudioBundle, AudioSource, Commands, Handle, Input, KeyCode, Local,
    Res, ResMut, Resource, Startup, Time, Timer, TimerMode, Update,
};
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;

use crate::ui::ui_system;

mod ui;

#[derive(Resource)]
struct Sounds {
    click: Handle<AudioSource>,
}

#[derive(Resource)]
struct ClickTimer(Timer);

#[derive(Resource, Eq, PartialEq, Clone)]
struct Settings {
    bpm: u16,
    play: bool,
    max_beats: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            bpm: DEFAULT_BPM,
            play: false,
            max_beats: 4,
        }
    }
}

#[derive(Resource, Eq, PartialEq, Clone, Default)]
struct State {
    beat: u8,
}

const DEFAULT_BPM: u16 = 120;
const MIN_BPM: u16 = 60;
const MAX_BPM: u16 = 300;

impl Default for ClickTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(seconds_from_bpm(DEFAULT_BPM), TimerMode::Repeating);
        timer.pause();
        Self(timer)
    }
}

fn seconds_from_bpm(bpm: u16) -> f32 {
    60. / bpm as f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<ClickTimer>()
        .init_resource::<Settings>()
        .init_resource::<State>()
        .add_systems(Startup, setup)
        .add_systems(Update, click_system)
        .add_systems(Update, ui_system)
        .add_systems(Update, update_system)
        .add_systems(Update, bpm_limit_system)
        .add_systems(Update, keyboard_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Sounds {
        click: asset_server.load("click.mp3"),
    });
}

fn click_system(
    time: Res<Time>,
    mut click_timer: ResMut<ClickTimer>,
    sounds: Res<Sounds>,
    mut commands: Commands,
    settings: Res<Settings>,
    mut state: ResMut<State>,
) {
    click_timer.0.tick(time.delta());
    if click_timer.0.just_finished() {
        state.beat = (state.beat + 1) % settings.max_beats;
        // TODO other click if beat == 1
        commands.spawn(AudioBundle {
            source: sounds.click.clone(),
            ..default()
        });
    }
}

fn update_system(
    settings: ResMut<Settings>,
    mut click_timer: ResMut<ClickTimer>,
    mut change_detector: Local<Settings>,
) {
    if *change_detector == *settings {
        return;
    }

    if settings.bpm != change_detector.bpm {
        click_timer
            .0
            .set_duration(Duration::from_secs_f32(seconds_from_bpm(settings.bpm)));
    }

    if settings.play != change_detector.play {
        if settings.play {
            click_timer.0.unpause();
            click_timer.0.reset();
            let duration = click_timer.0.duration() - Duration::from_micros(1);
            click_timer.0.tick(duration);
        } else {
            click_timer.0.pause();
        }
    }

    *change_detector = settings.clone();
}

fn keyboard_system(keyboard_input: Res<Input<KeyCode>>, mut settings: ResMut<Settings>) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Return) {
        settings.play = !settings.play;
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        settings.bpm += 1;
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        settings.bpm -= 1;
    }

    if keyboard_input.just_pressed(KeyCode::Left) {
        settings.bpm -= 10;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        settings.bpm += 10;
    }
}

fn bpm_limit_system(mut settings: ResMut<Settings>) {
    if settings.bpm < MIN_BPM {
        settings.bpm = MIN_BPM;
    }

    if settings.bpm > MAX_BPM {
        settings.bpm = MAX_BPM;
    }
}
