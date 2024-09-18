use crate::screen::main::state::ScreenMainState;
use crate::screen::main::ui::*;
use crate::state::GameState;
use bevy::app::{App, Plugin};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Update};

pub struct ScreenMainPlugin;

impl Plugin for ScreenMainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenMainState::default())
            .add_systems(OnEnter(GameState::Main), on_enter)
            .add_systems(OnExit(GameState::Main), on_exit)
            .add_systems(Update, on_press.run_if(in_state(GameState::Main)))
            .observe(on_select_editor);
    }
}
