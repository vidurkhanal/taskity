mod app;
mod constants;
mod event;
mod root;
mod tabs;
mod tui;
mod ui;
mod update;

use std::sync::{atomic::AtomicBool, Arc, Mutex};

use app::App;
use color_eyre::eyre::Result;
use event::{Event, EventHandler};
use ratatui::{prelude::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    // Create an application.

    let should_quit = Arc::new(AtomicBool::new(false));
    let mut app = Arc::new(Mutex::new(App::new(should_quit)));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let app_clone = Arc::clone(&app);
    let should_quit_clone = Arc::clone(&app).lock().unwrap().should_quit.clone();

    std::thread::spawn(move || {
        while !should_quit_clone.load(std::sync::atomic::Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_secs(1));
            app_clone.lock().unwrap().context.system.refresh_processes();
        }
    });

    // Start the main loop.
    while !app
        .lock()
        .unwrap()
        .should_quit
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        // Render the user interface.
        tui.draw(&mut app.lock().unwrap())?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app.lock().unwrap(), key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
