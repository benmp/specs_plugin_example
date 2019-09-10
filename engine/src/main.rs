// use crossterm::{input, AlternateScreen, ClearType, Crossterm, InputEvent, KeyEvent, RawScreen};

use specs::prelude::*;

use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Press 'ESC' to quit.");

    //RENDERING
    // AlternateScreen::to_alternate(false)?;

    // let crossterm = Crossterm::new();
    // let terminal = crossterm.terminal();
    // let cursor = crossterm.cursor();

    // terminal.clear(ClearType::All).unwrap();

    // cursor.goto(0, 0).unwrap();
    // cursor.hide().unwrap();

    // RawScreen::into_raw_mode()?;
    // let input = input();

    // let mut async_stdin = input.read_async();

    //PLUGINS

    //SPECS WORLD
    let mut world = World::new();

    world.register::<components::Character>();
    world
        .create_entity()
        .with(components::Character('s'))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(charr::RenderChar::new(), "charr", &[])
        .build();

    dispatcher.setup(&mut world);

    dispatcher.dispatch(&world);

    println!("Press 'ESC' to quit.");

    //GAME LOOP SETUP
    //let mut framelimiter =
    //    framelimiter::FrameLimiter::new(framelimiter::FrameRateLimitStrategy::Sleep, 60);

    //let ticks_per_second = 120u64;
    //let delta_time = 1_000_000_000u64 / ticks_per_second;

    ////initialize to run one loop
    //let mut accumulator = 0u64;

    //// start accumulated game time
    //let mut total_tick_time = 0u64;
    //let mut start_current_loop = std::time::Instant::now();

    'running: loop {
        dispatcher.dispatch(&world);

        std::thread::sleep(Duration::from_secs(5));

        //    //INPUT
        //    // collect input on previous loop timer so that when we tick it includes all events
        //    while let Some(key_event) = async_stdin.next() {
        //        //TODO queue events into buffer using std::time::Instant::now.duration_since(total_tick_time)
        //        //TODO this way we'll get timestamps (100ms to 200ms held down etc..) and can compare to
        //        // sliding window of total_tick_time to total_tick_time + delta_time
        //        match key_event {
        //            InputEvent::Keyboard(event) => match event {
        //                KeyEvent::Esc => break 'running,
        //                KeyEvent::Char('J') => println!("Key {:?} was pressed J!", event),
        //                _ => println!("Key {:?} was pressed!", event),
        //            },
        //            InputEvent::Mouse(_event) => { /* Mouse Event */ }
        //            _ => {}
        //        }
        //    }

        //    //stop previous loop, then start next instantly
        //    let stop_current_loop = std::time::Instant::now();
        //    let start_next_loop = std::time::Instant::now();

        //    // add elapsed time to accumulator
        //    accumulator += u64::try_from(
        //        stop_current_loop
        //            .duration_since(start_current_loop)
        //            .as_nanos(),
        //    )?;

        //    // reset using temporary variable
        //    start_current_loop = start_next_loop;

        //    // at least one whole physics step to compute
        //    while accumulator >= delta_time {
        //        //dispatch systems
        //        dispatcher.dispatch(&mut world);
        //        //input system needs to know overall timeframe for events
        //        //sliding window start = total_tick_time
        //        //sliding window end = total_tick_time + delta_time

        //        //assuming were not doing EVE online stuff with time dialation, no need to pass in delta_time to systems?

        //        accumulator -= delta_time;
        //        total_tick_time += delta_time;
        //    }

        //    let interpolation_alpha = accumulator as f64 / delta_time as f64;

        //    //dispatch render system using interpolation_alpha

        //    //let this nice helper class clamp rendering framerate for us
        //    framelimiter.wait();
    }

    // println!("Program closing ...");

    // Ok(())
}
