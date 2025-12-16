use iced::{Color, Theme};
use crate::ui::colors::*;

// Helper functions to create custom styles
// In iced 0.13.1, .style() expects functions that receive (theme, status)

// Purple border, purple text, black background
pub fn purple_button_style(_theme: &Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    let border_color = match status {
        iced::widget::button::Status::Pressed => ACCENT_PURPLE_PRESSED,
        iced::widget::button::Status::Hovered => ACCENT_PURPLE_HOVER,
        _ => ACCENT_PURPLE,
    };
    iced::widget::button::Style {
        background: Some(iced::Background::Color(BG_DARK)), // Black background
        text_color: border_color, // Purple text
        border: iced::Border {
            color: border_color, // Purple border
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: iced::Shadow::default(),
    }
}

pub fn dark_container_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(BG_DARK)),
        text_color: Some(TEXT_LIGHT),
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
    }
}

pub fn text_light_style(_theme: &Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(TEXT_LIGHT),
    }
}

pub fn purple_checkbox_style(_theme: &Theme, status: iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
    let (border_color, border_width) = match status {
        iced::widget::checkbox::Status::Active { is_checked: true } => (ACCENT_PURPLE, 2.0),
        iced::widget::checkbox::Status::Hovered { is_checked: true } => (ACCENT_PURPLE_HOVER, 2.0),
        iced::widget::checkbox::Status::Hovered { is_checked: false } => (ACCENT_PURPLE_HOVER, 1.0),
        _ => (BORDER_GRAY, 1.0),
    };
    
    iced::widget::checkbox::Style {
        background: iced::Background::Color(BG_INPUT),
        icon_color: Color::WHITE,
        border: iced::Border {
            color: border_color,
            width: border_width,
            radius: 4.0.into(),
        },
        text_color: Some(TEXT_LIGHT),
    }
}

// Dark background, purple border always
pub fn dark_text_input_style(_theme: &Theme, status: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    let (border_color, border_width) = match status {
        iced::widget::text_input::Status::Focused => (ACCENT_PURPLE_HOVER, 2.0), // Lighter border when focused
        _ => (ACCENT_PURPLE, 1.0), // Purple border always
    };
    
    iced::widget::text_input::Style {
        background: iced::Background::Color(BG_INPUT),
        border: iced::Border {
            color: border_color,
            width: border_width,
            radius: 4.0.into(),
        },
        icon: TEXT_LIGHT, // icon is a Color in iced 0.13.1
        placeholder: Color::from_rgb(0x80 as f32 / 255.0, 0x80 as f32 / 255.0, 0x80 as f32 / 255.0),
        selection: ACCENT_PURPLE,
        value: TEXT_LIGHT,
    }
}

// Style for text_input with error (red border)
pub fn error_text_input_style(_theme: &Theme, status: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    let (border_color, border_width) = match status {
        iced::widget::text_input::Status::Focused => (ERROR_RED, 2.0), // Thicker red border when focused
        _ => (ERROR_RED, 2.0), // Red border always
    };
    
    iced::widget::text_input::Style {
        background: iced::Background::Color(BG_INPUT),
        border: iced::Border {
            color: border_color,
            width: border_width,
            radius: 4.0.into(),
        },
        icon: ERROR_RED, // icon in red
        placeholder: ERROR_RED, // placeholder in red
        selection: ERROR_RED,
        value: ERROR_RED, // text in red
    }
}

