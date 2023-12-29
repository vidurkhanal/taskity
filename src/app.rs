pub struct App {
    pub should_quit: bool,
    pub context: AppContext,
}

#[derive(Debug, Default, Clone)]
pub struct AppContext {
    pub tab_index: usize,
    pub row_index: usize,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            should_quit: false,
            context: AppContext {
                tab_index: 0,
                row_index: 0,
            },
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
