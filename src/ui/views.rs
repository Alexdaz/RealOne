use iced::{widget::{button, checkbox, column, container, row, text, text_input, Column, scrollable, pick_list}, Element, Length};
use crate::state::HashFormat;
use crate::state::RealOne;
use crate::state::Message;
use crate::ui::styles::*;
use crate::hash::{Algorithm, GostVariant, TigerVariant};

pub fn view_main(state: &RealOne) -> Element<'_, Message, iced::Theme> {
    let file_path_text = if let Some(ref error) = state.file_error {
        error.clone()
    } else {
        state
            .file_path
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("No file selected")
            .to_string()
    };

    let file_input_style = if state.file_error.is_some() {
        error_text_input_style
    } else {
        dark_text_input_style
    };

    let file_input = text_input("Select a file...", &file_path_text)
        .size(16)
        .width(Length::Fill)
        .style(file_input_style);

    let file_row = row![
        text("File:")
            .size(16)
            .style(text_light_style),
        file_input.width(Length::Fill),
        button("Browse...")
            .on_press(Message::BrowseFile)
            .style(purple_button_style)
    ]
    .spacing(10);

    let check_hash_input = text_input("Paste hash to compare...", &state.check_hash)
        .size(16)
        .width(Length::Fill)
        .style(dark_text_input_style)
        .on_input(Message::CheckHashChanged);

    let check_row = row![
        text("Check:")
            .size(16)
            .style(text_light_style),
        check_hash_input.width(Length::Fill)
    ]
    .spacing(10);

    let check_button = button("Check")
        .on_press(Message::CheckButtonPressed)
        .style(purple_button_style)
        .width(Length::Shrink);

    let settings_button = button("Settings")
        .on_press(Message::SettingsButtonPressed)
        .style(purple_button_style)
        .width(Length::Shrink);

    let buttons_row = row![check_button, settings_button]
        .spacing(10);

    let mut results_column = Column::new()
        .spacing(10)
        .padding(10);

    for algorithm in &state.selected_algorithms {
        let hash_value = state
            .hash_results
            .get(algorithm)
            .cloned()
            .unwrap_or_else(|| String::new());

        // Normalize hashes for comparison
        // Convert both to bytes to compare independently of format
        use base64::Engine;
        let check_hash_bytes = if let Ok(bytes) = hex::decode(state.check_hash.trim().replace(' ', "")) {
            Some(bytes)
        } else if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(state.check_hash.trim().replace(' ', "")) {
            Some(bytes)
        } else {
            None
        };
        
        let calculated_hash_bytes = if let Ok(bytes) = hex::decode(hash_value.trim().replace(' ', "")) {
            Some(bytes)
        } else if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(hash_value.trim().replace(' ', "")) {
            Some(bytes)
        } else {
            None
        };
        
        // Compare bytes directly
        let matches = check_hash_bytes.is_some()
            && calculated_hash_bytes.is_some()
            && !hash_value.is_empty()
            && hash_value != "Error: Not implemented"
            && check_hash_bytes == calculated_hash_bytes;

        let algorithm_name = format!("{}:", algorithm.to_string());
        let hash_input = text_input("", &hash_value)
            .size(14)
            .width(Length::Fill)
            .style(dark_text_input_style);

        let algorithm_clone = algorithm.clone();
        let copy_button = button("Copy")
            .on_press(Message::CopyHash(algorithm_clone))
            .style(purple_button_style)
            .width(Length::Shrink);

        // Create row with hash, copy button and check emoji if it matches
        let mut result_row = row![
            text(algorithm_name)
                .size(14)
                .width(Length::Shrink)
                .style(text_light_style),
            hash_input,
            copy_button
        ]
        .spacing(10);

        // Add green match indicator if they match
        if matches {
            result_row = result_row.push(
                container(
                    text("[MATCH]")
                        .size(12)
                        .style(|_theme: &iced::Theme| {
                            iced::widget::text::Style {
                                color: Some(iced::Color::from_rgb(0x49 as f32 / 255.0, 0xEB as f32 / 255.0, 0x7A as f32 / 255.0)), // Green
                            }
                        })
                )
                .padding(4)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .align_y(iced::alignment::Vertical::Center)
            );
        }

        results_column = results_column.push(result_row);
    }

    // Show "Computing..." label if there's a calculation in progress
    let mut progress_section = Column::new().spacing(5);
    if state.calculation_start.is_some() {
        progress_section = progress_section.push(
            text("Computing...")
                .size(14)
                .style(text_light_style)
        );
    }

    let content = column![
        file_row,
        check_row,
        buttons_row,
        text("Hash Results:")
            .size(16)
            .style(text_light_style),
        results_column,
        progress_section
    ]
    .spacing(15)
    .padding(20)
    .width(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(dark_container_style)
        .into()
}

pub fn view_settings(state: &RealOne) -> Element<'_, Message, iced::Theme> {
    let all_algorithms = Algorithm::all();
    
    // Combo boxes for result format, GOST variant, and TIGER variant - move them to the top
    let format_label = text("Result Format:")
        .size(16)
        .style(text_light_style);
    
    let format_picker = pick_list(
        HashFormat::all(),
        Some(state.hash_format),
        Message::FormatChanged,
    )
    .width(Length::Fill)
    .style(|_theme: &iced::Theme, status| {
        use crate::ui::colors::*;
        iced::widget::pick_list::Style {
            text_color: TEXT_LIGHT,
            background: iced::Background::Color(BG_INPUT),
            border: iced::Border {
                color: match status {
                    iced::widget::pick_list::Status::Hovered => ACCENT_PURPLE_HOVER,
                    _ => ACCENT_PURPLE,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            placeholder_color: TEXT_LIGHT,
            handle_color: ACCENT_PURPLE,
        }
    });

    let gost_label = text("GOST Variant:")
        .size(16)
        .style(text_light_style);
    
    let gost_picker = pick_list(
        GostVariant::all(),
        Some(state.gost_variant),
        Message::GostVariantChanged,
    )
    .width(Length::Fill)
    .style(|_theme: &iced::Theme, status| {
        use crate::ui::colors::*;
        iced::widget::pick_list::Style {
            text_color: TEXT_LIGHT,
            background: iced::Background::Color(BG_INPUT),
            border: iced::Border {
                color: match status {
                    iced::widget::pick_list::Status::Hovered => ACCENT_PURPLE_HOVER,
                    _ => ACCENT_PURPLE,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            placeholder_color: TEXT_LIGHT,
            handle_color: ACCENT_PURPLE,
        }
    });

    let tiger_label = text("TIGER Variant:")
        .size(16)
        .style(text_light_style);
    
    let tiger_picker = pick_list(
        TigerVariant::all(),
        Some(state.tiger_variant),
        Message::TigerVariantChanged,
    )
    .width(Length::Fill)
    .style(|_theme: &iced::Theme, status| {
        use crate::ui::colors::*;
        iced::widget::pick_list::Style {
            text_color: TEXT_LIGHT,
            background: iced::Background::Color(BG_INPUT),
            border: iced::Border {
                color: match status {
                    iced::widget::pick_list::Status::Hovered => ACCENT_PURPLE_HOVER,
                    _ => ACCENT_PURPLE,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            placeholder_color: TEXT_LIGHT,
            handle_color: ACCENT_PURPLE,
        }
    });

    // Split algorithms into two columns
    let mid_point = (all_algorithms.len() + 1) / 2; // Split approximately in half
    let (left_algorithms, right_algorithms) = all_algorithms.split_at(mid_point);

    // Left column
    let mut left_column = Column::new()
        .spacing(10)
        .padding(10)
        .width(Length::Fill);

    for algorithm in left_algorithms {
        let is_selected = state.selected_algorithms.contains(algorithm);
        let alg_clone = algorithm.clone();
        let checkbox_widget = checkbox(
            algorithm.to_string(),
            is_selected,
        )
        .style(purple_checkbox_style)
        .on_toggle(move |enabled| Message::AlgorithmToggled(alg_clone.clone(), enabled));
        left_column = left_column.push(checkbox_widget);
    }

    // Right column
    let mut right_column = Column::new()
        .spacing(10)
        .padding(10)
        .width(Length::Fill);

    for algorithm in right_algorithms {
        let is_selected = state.selected_algorithms.contains(algorithm);
        let alg_clone = algorithm.clone();
        let checkbox_widget = checkbox(
            algorithm.to_string(),
            is_selected,
        )
        .style(purple_checkbox_style)
        .on_toggle(move |enabled| Message::AlgorithmToggled(alg_clone.clone(), enabled));
        right_column = right_column.push(checkbox_widget);
    }

    // Create the two columns side by side
    let checkboxes_row = row![
        left_column,
        right_column
    ]
    .spacing(20)
    .width(Length::Fill);

    let scrollable_content = scrollable(
        checkboxes_row
            .width(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill);

    let content = column![
        format_label,
        format_picker,
        gost_label,
        gost_picker,
        tiger_label,
        tiger_picker,
        text("Select Hash Algorithms:")
            .size(18)
            .style(text_light_style),
        scrollable_content
    ]
    .spacing(15)
    .width(Length::Fill)
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(dark_container_style)
        .into()
}

