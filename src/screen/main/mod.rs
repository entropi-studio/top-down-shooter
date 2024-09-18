use crate::state::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, Commands, IntoSystemConfigs, NextState, ResMut, Startup};
use bevy_egui::{egui, EguiContexts};
use ui_toolkit::dialog::{ToolkitDialog, ToolkitDialogType, ToolkitDialogOpenTrigger};
use ui_toolkit::prelude::{ToolkitDialogBuilder, ToolkitDialogCloseAllTrigger};

pub struct ScreenMainPlugin;

impl Plugin for ScreenMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, draw_ui.run_if(in_state(GameState::Main)));
    }
}

fn init(mut commands: Commands) {
    ToolkitDialogBuilder::new()
        .title("Title")
        .dialog_type(ToolkitDialogType::alert_builder().content("Hello?").build())
        .build()
        .open(&mut commands);
}

fn draw_ui(
    mut state: ResMut<NextState<GameState>>,
    mut contexts: EguiContexts,
    mut commands: Commands,
) {
    if contexts.try_ctx_mut().is_none() {
        return;
    }

    egui::Window::new("Top Down Shooter").show(contexts.ctx_mut(), |ui| {
        let _ = ui.button("Start");
        if ui.button("Editor").clicked() {
            state.set(GameState::Editor);
            commands.trigger(ToolkitDialogCloseAllTrigger);
        }
        if ui.button("Dialog?").clicked() {
            ToolkitDialogBuilder::new()
                .title("Shit")
                .dialog_type(
                    ToolkitDialogType::alert_builder()
                        .content(rand::random::<i32>().to_string())
                        .build(),
                )
                .build()
                .open(&mut commands);
        }
        let _ = ui.button("Quit");
    });
}
