use bevy::prelude::{Res, ResMut};
use bevy_egui::egui::{Ui, Widget};
use bevy_egui::{egui, EguiContexts};
use egui::widgets::Button;

use crate::{Settings, State, MAX_BPM, MIN_BPM};

pub fn ui_system(mut contexts: EguiContexts, mut settings: ResMut<Settings>, state: Res<State>) {
    egui::Area::new("metronome").show(contexts.ctx_mut(), |ui| {
        ui.heading("Metronome");
        ui.separator();
        ui.horizontal(|ui| {
            bpm_controls(ui, &mut settings);
        });
        ui.separator();

        ui.label(format!("Beat: {}", state.beat));

        play_button(settings, ui);
    });
}

fn bpm_controls(ui: &mut Ui, settings: &mut ResMut<Settings>) {
    ui.label("BPM");
    ui.add(egui::Slider::new(&mut settings.bpm, MIN_BPM..=MAX_BPM));

    ui.vertical(|ui| {
        bpm_control_button(
            ui,
            "+",
            || settings.bpm += 1,
            "Decrease BPM by 1 (Arrow Down)",
        );
        bpm_control_button(
            ui,
            "-",
            || settings.bpm -= 1,
            "Decrease BPM by 10 (Arrow Left)",
        );
    });

    ui.vertical(|ui| {
        bpm_control_button(
            ui,
            "+10",
            || settings.bpm += 10,
            "Increase BPM by 10 (Arrow Right)",
        );
        bpm_control_button(
            ui,
            "-10",
            || settings.bpm -= 10,
            "Decrease BPM by 10 (Arrow Left)",
        );
    });
}

fn bpm_control_button<F: FnMut()>(ui: &mut Ui, text: &str, mut bpm_delta: F, tooltip: &str) {
    ui.horizontal(|ui| {
        let button = ui
            .add(Button::new(text).min_size([20., 10.].into()))
            .on_hover_text(tooltip);
        if button.clicked() {
            bpm_delta();
        }
    });
}

fn play_button(mut settings: ResMut<Settings>, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let play_button = Button::new(match settings.play {
            true => "Stop",
            false => "Play",
        })
        .min_size([100., 50.].into())
        .ui(ui)
        .on_hover_text("(Space/Enter)");

        if play_button.clicked() {
            settings.play = !settings.play;
        }
    });
}
