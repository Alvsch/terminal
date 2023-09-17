use std::collections::HashMap;
use crate::command::Command;

pub struct CommandDispatcher {
    commands: HashMap<String, Command>,
}

impl CommandDispatcher {
    pub(crate) fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn get_command(&self, name: &String) -> Option<&Command> {
        self.commands.get(name)
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.insert(command.get_name(), command);
    }

}
