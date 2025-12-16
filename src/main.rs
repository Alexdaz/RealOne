mod state;
mod app;
mod config;
mod ui;
mod hash;

use state::RealOne;
use state::Message;
use ui::windows::MainWindow;
use app::{update, view, title, subscription};
use config::load_config;

fn main() -> iced::Result {
    // Load saved configuration
    let (saved_algorithms, saved_format, saved_gost_variant, saved_tiger_variant) = load_config();
    
    let mut state = RealOne::default();
    state.selected_algorithms = saved_algorithms;
    state.hash_format = saved_format;
    state.gost_variant = saved_gost_variant;
    state.tiger_variant = saved_tiger_variant;
    
    // Open main window at startup using WindowManager
    let (_main_window_id, main_window_task) = state.window_manager.open(Box::new(MainWindow));
    
    // Convert Task<Id> to Task<Message> - ignore result since we only need to open the window
    let init_task = main_window_task.map(|_id| {
        Message::WindowClosed(iced::window::Id::unique())
    });
    
    iced::daemon(title, update, view)
        .subscription(subscription)
        .run_with(|| (state, init_task))
}
