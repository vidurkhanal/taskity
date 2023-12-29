use std::sync::{atomic::AtomicBool, Arc};

use sysinfo::System;

pub struct App {
    pub should_quit: Arc<AtomicBool>,
    pub context: AppContext,
}

#[derive(Debug, Default)]
pub struct AppContext {
    pub tab_index: usize,
    pub row_index: usize,
    pub system: System,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(should_quit: Arc<AtomicBool>) -> Self {
        Self {
            should_quit,
            context: AppContext {
                tab_index: 0,
                row_index: 0,
                system: System::new_all(),
            },
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
