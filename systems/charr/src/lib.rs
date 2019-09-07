use dynamic_reload::{DynamicReload, Lib, PlatformName, Search, Symbol, UpdateState};
use specs::prelude::*;
extern crate components;
extern crate plugins;

// DynamicReload::new(Some(vec!["target/debug"]), Some("/tmp"), Search::Default);
pub struct RenderChar {
    pub reload_handler: DynamicReload,
    pub plugins: plugins::Plugins,
}

impl RenderChar {
    pub fn new() -> RenderChar {
        RenderChar {
            reload_handler: DynamicReload::new(
                Some(vec!["target/debug"]),
                Some("/tmp"),
                Search::Default,
            ),
            plugins: plugins::Plugins {
                plugins: Vec::new(),
            },
        }
    }
}

impl<'a> System<'a> for RenderChar {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = ReadStorage<'a, components::Character>;

    fn run(&mut self, mut storage: Self::SystemData) {
        for mychar in (&mut storage).join() {
            //mychar.0 = 'a';
            println!("{}", mychar.0);
        }

        self.reload_handler
            .update(plugins::Plugins::reload_callback, &mut self.plugins);

        //TODO figure out how to cache this symbol?
        let fun: Symbol<fn() -> i32> =
            unsafe { self.plugins.plugins[0].lib.get(b"it_works").unwrap() };
        println!("{}", fun());
    }

    fn setup(&mut self, res: &mut specs::prelude::World) {
        Self::SystemData::setup(res);

        match self
            .reload_handler
            .add_library("libcharr_dylib.so", PlatformName::No)
        {
            Ok(lib) => self.plugins.add_plugin(&lib),
            Err(e) => {
                println!("Unable to load dynamic lib, err {:?}", e);
                return;
            }
        }
    }
}
