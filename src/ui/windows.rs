use iced_multi_window::Window;
use crate::state::{RealOne, Message};
use crate::ui::theme::custom_dark_theme;
use crate::ui::views::{view_main, view_settings};

#[derive(Debug, Clone)]
pub struct MainWindow;

impl Window<RealOne, iced::Theme, Message> for MainWindow {
    fn view<'a>(&'a self, app: &'a RealOne) -> iced::Element<'a, Message, iced::Theme> {
        view_main(app)
    }

    fn title(&self, _app: &RealOne) -> String {
        "Real One - File Hash Checker".to_string()
    }

    fn theme(&self, _app: &RealOne) -> iced::Theme {
        custom_dark_theme()
    }

    fn settings(&self) -> iced::window::Settings {
        iced::window::Settings {
            size: iced::Size::new(800.0, 410.0),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsWindow;

impl Window<RealOne, iced::Theme, Message> for SettingsWindow {
    fn view<'a>(&'a self, app: &'a RealOne) -> iced::Element<'a, Message, iced::Theme> {
        view_settings(app)
    }

    fn title(&self, _app: &RealOne) -> String {
        "Settings - Real One".to_string()
    }

    fn theme(&self, _app: &RealOne) -> iced::Theme {
        custom_dark_theme()
    }

    fn settings(&self) -> iced::window::Settings {
        iced::window::Settings {
            size: iced::Size::new(600.0, 520.0), // Width increased for 2 columns
            ..Default::default()
        }
    }
}

