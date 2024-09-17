use crate::command::ToolkitCommand;
use bevy::ecs::system::SystemParam;
use bevy::prelude::Resource;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct ToolkitContext {
    pub(crate) commands: HashMap<u128, ToolkitCommand>,
}

impl ToolkitContext {
    fn generate_id(&self) -> u128 {
        let generated = rand::random::<u128>();
        if self.commands.contains_key(&generated) {
            self.generate_id()
        } else {
            generated
        }
    }

    pub fn command(&mut self, command: ToolkitCommand) {
        self.commands.insert(self.generate_id(), command);
    }
}
