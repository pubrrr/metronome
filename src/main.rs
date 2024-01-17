use std::time::Duration;

use bevy::prelude::{
    default, App, AssetServer, AudioBundle, AudioSource, Commands, Handle, Input, KeyCode, Local,
    Res, ResMut, Resource, Startup, Time, Timer, TimerMode, Update,
};
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

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
}

const DEFAULT_BPM: u16 = 120;

impl Default for Settings {
    fn default() -> Self {
        Self {
            bpm: DEFAULT_BPM,
            play: false,
        }
    }
}

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
        .add_systems(Startup, setup)
        .add_systems(Update, click_system)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, update_system)
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
) {
    click_timer.0.tick(time.delta());
    if click_timer.0.just_finished() {
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

    if settings.play != change_detector.play {
        if settings.play {
            click_timer.0.unpause();
            click_timer.0.reset();
        } else {
            click_timer.0.pause();
        }
    }

    if settings.bpm != change_detector.bpm {
        click_timer
            .0
            .set_duration(Duration::from_secs_f32(seconds_from_bpm(settings.bpm)));
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

fn ui_example_system(mut contexts: EguiContexts, mut settings: ResMut<Settings>) {
    egui::Area::new("metronome").show(contexts.ctx_mut(), |ui| {
        ui.label("Metronome");
        ui.separator();
        ui.label("BPM");
        ui.horizontal_wrapped(|ui| {
            ui.centered_and_justified(|ui| {
                ui.add(egui::Slider::new(&mut settings.bpm, 60..=300).text("BPM"));
            });
            ui.vertical(|ui| {
                if ui
                    .button("+")
                    .on_hover_text("Increase BPM by 1 (Arrow Up)")
                    .clicked()
                {
                    settings.bpm += 1;
                }
                if ui
                    .button("-")
                    .on_hover_text("Decrease BPM by 1 (Arrow Down)")
                    .clicked()
                {
                    settings.bpm -= 1;
                }
            });
            ui.vertical(|ui| {
                if ui
                    .button("+10")
                    .on_hover_text("Increase BPM by 10 (Arrow Right)")
                    .clicked()
                {
                    settings.bpm += 10;
                }
                if ui
                    .button("-10")
                    .on_hover_text("Decrease BPM by 10 (Arrow Left)")
                    .clicked()
                {
                    settings.bpm -= 10;
                }
            });
        });
        ui.separator();
        ui.checkbox(&mut settings.play, "Play");
    });
}
