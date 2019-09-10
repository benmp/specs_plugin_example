use dynamic_reload::{Lib, UpdateState};
use std::sync::Arc;

pub struct Plugin {
    pub plugin: Option<Arc<Lib>>,
}

impl Plugin {
    pub fn set_plugin(&mut self, plugin: &Arc<Lib>) {
        self.plugin = Some(Arc::clone(plugin));
    }

    pub fn remove_plugin(&mut self) {
        self.plugin = None;
    }

    // called when a lib needs to be reloaded.
    pub fn reload_callback(&mut self, state: UpdateState, lib: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => Self::remove_plugin(self),
            UpdateState::After => {
                println!("reloaded");
                Self::set_plugin(self, lib.unwrap())
            }
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }
}
