use dynamic_reload::{DynamicReload, PlatformName, Search, Symbol};
use specs::prelude::*;

pub struct RenderChar {
    pub reload_handler: DynamicReload,
    pub plugin: plugin::Plugin,
}

impl RenderChar {
    pub fn new() -> RenderChar {
        RenderChar {
            reload_handler: DynamicReload::new(
                Some(vec!["target/debug"]),
                Some("/tmp"),
                Search::Default,
            ),
            plugin: plugin::Plugin { plugin: None },
        }
    }
}

impl Default for RenderChar {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> System<'a> for RenderChar {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = ReadStorage<'a, components::Character>;

    fn run(&mut self, storage: Self::SystemData) {
        for mychar in (storage).join() {
            //mychar.0 = 'a';
            println!("{}", mychar.0);
        }

        self.reload_handler
            .update(plugin::Plugin::reload_callback, &mut self.plugin);

        //TODO figure out how to cache this symbol?
        let fun: Symbol<fn() -> i32> = unsafe {
            self.plugin
                .plugin
                .as_ref()
                .unwrap()
                .lib
                .get(b"it_works")
                .unwrap()
        };
        println!("{}", fun());
    }

    fn setup(&mut self, res: &mut specs::prelude::World) {
        Self::SystemData::setup(res);

        match self
            .reload_handler
            .add_library("libcharr_dylib.so", PlatformName::No)
        {
            Ok(lib) => self.plugin.set_plugin(&lib),
            Err(e) => {
                println!("Unable to load dynamic lib, err {:?}", e);
            }
        }
    }
}
