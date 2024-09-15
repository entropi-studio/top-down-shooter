use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, NextState, Res, ResMut, State};
use bevy_egui::{egui, EguiContexts};
use crate::state::GameState;

pub struct ScreenMainPlugin;

impl Plugin for ScreenMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_ui.run_if(in_state(GameState::Main)));
    }
}

fn draw_ui(mut state: ResMut<NextState<GameState>>, mut contexts: EguiContexts) {
    egui::Window::new("Top Down Shooter").show(contexts.ctx_mut(), |ui| {
        ui.button("Start");
        if ui.button("Editor").clicked() {
            state.set(GameState::Editor);
        }
        ui.button("Quit");
    });
}