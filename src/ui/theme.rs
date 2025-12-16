use iced::{Color, Theme};
use std::sync::Arc;
use crate::ui::colors::*;

// Custom theme with dark mode and purple accent
pub fn custom_dark_theme() -> Theme {
    use iced::theme::{Custom, Palette};
    
    let palette = Palette {
        background: BG_DARK,
        text: TEXT_LIGHT,
        primary: ACCENT_PURPLE,
        success: Color::from_rgb(0x49 as f32 / 255.0, 0xEB as f32 / 255.0, 0x7A as f32 / 255.0), // #49EB7A - green for success
        danger: Color::from_rgb(0xFF as f32 / 255.0, 0x55 as f32 / 255.0, 0x55 as f32 / 255.0), // #FF5555 - red for errors
    };
    
    Theme::Custom(Arc::new(Custom::new("Dark Purple".to_string(), palette)))
}

