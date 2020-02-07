use crossterm::{AlternateScreen, ClearType, Crossterm, Terminal, TerminalCursor};
use dynamic_reload::{DynamicReload, PlatformName, Search, Symbol};
use specs::prelude::*;
use std::io::{stdout, Write};

pub struct RenderChar {
    pub reload_handler: DynamicReload,
    pub plugin: plugin::Plugin,
    pub terminal: Terminal,
    pub cursor: TerminalCursor,
}

impl RenderChar {
    pub fn new() -> RenderChar {
        let crossterm = Crossterm::new();
        let terminal = crossterm.terminal();
        let cursor = crossterm.cursor();

        terminal.clear(ClearType::All).unwrap();
        AlternateScreen::to_alternate(true).unwrap();

        RenderChar {
            reload_handler: DynamicReload::new(
                Some(vec!["target/debug"]),
                Some("/tmp"),
                Search::Default,
            ),
            plugin: plugin::Plugin { plugin: None },
            terminal,
            cursor,
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
    type SystemData = (
        ReadStorage<'a, ecs_impl::WCharacter>,
        WriteStorage<'a, ecs_impl::WPosition>,
    );

    fn run(&mut self, (character, mut position): Self::SystemData) {
        self.reload_handler
            .update(plugin::Plugin::reload_callback, &mut self.plugin);

        let fun: Symbol<
            fn(&mut TerminalCursor, &components::Character, &mut components::Position),
        > = unsafe {
            self.plugin
                .plugin
                .as_ref()
                .unwrap()
                .lib
                .get(b"it_works")
                .unwrap()
        };

        self.terminal.clear(ClearType::All).unwrap();

        for (character, position) in (&character, &mut position).join() {
            fun(&mut self.cursor, &character.0, &mut position.0);
        }

        stdout().flush().unwrap();
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

        //RENDERING
        self.terminal.clear(ClearType::All).unwrap();

        self.cursor.goto(0, 0).unwrap();
        self.cursor.hide().unwrap();
    }
}
