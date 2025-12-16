use iced::{Element, Subscription, Task};
use iced_multi_window::Window;
use crate::state::{RealOne, Message};
use crate::ui::windows::{MainWindow, SettingsWindow};
use crate::hash::calculate_hashes_parallel_streaming;
use crate::ui::views;
use crate::config::save_config;
use std::time::Instant;
use std::collections::HashMap;
use std::process;

pub fn update(state: &mut RealOne, message: Message) -> Task<Message> {
    match message {
        Message::BrowseFile => {
            Task::perform(
                async {
                    rfd::AsyncFileDialog::new()
                        .pick_file()
                        .await
                        .map(|f| f.path().to_path_buf())
                },
                Message::FileSelected,
            )
        }
        Message::FileSelected(path) => {
            state.file_path = path;
            state.hash_results.clear();
            state.file_error = None; // Clear error when a file is selected
            Task::none()
        }
        Message::CheckHashChanged(hash) => {
            state.check_hash = hash;
            Task::none()
        }
        Message::CheckButtonPressed => {
            if let Some(ref path) = state.file_path {
                let algorithms = state.selected_algorithms.clone();
                let gost_variant = state.gost_variant;
                let tiger_variant = state.tiger_variant;
                let path_clone = path.clone();
                
                // Clear error if file exists
                state.file_error = None;
                
                // Start calculation - save start time
                state.calculation_start = Some(Instant::now());
                state.hash_results.clear(); // Clear previous results
                
                // OPTIMIZATION: Read file ONCE using streaming
                // Process all algorithms in a single pass, without loading everything into memory
                let algorithms_for_thread = algorithms.clone();
                let algorithms_for_error = algorithms.clone();
                
                Task::perform(
                    async move {
                        // Execute in a separate thread to avoid blocking the UI
                        std::thread::spawn(move || {
                            calculate_hashes_parallel_streaming(&path_clone, &algorithms_for_thread, gost_variant, tiger_variant)
                        })
                        .join()
                        .unwrap_or_else(|_| {
                            algorithms_for_error.iter().map(|alg| (alg.clone(), "Error: Thread panicked".to_string())).collect()
                        })
                    },
                    |results| Message::HashesCalculated(results),
                )
            } else {
                // No file selected - show error
                state.file_error = Some("Missing file".to_string());
                Task::none()
            }
        }
        Message::HashesCalculated(results) => {
            // Update all results at once
            // Hashes come in lowercase hex format; convert them to the selected format
            for (algorithm, hash_hex) in results {
                // Decode hex and convert to selected format
                if let Ok(hash_bytes) = hex::decode(&hash_hex) {
                    let formatted = state.hash_format.format_hash(&hash_bytes);
                    state.hash_results.insert(algorithm, formatted);
                } else {
                    // If not valid hex (may be "Not implemented"), keep it as is
                    state.hash_results.insert(algorithm, hash_hex);
                }
            }
            // Clear calculation state
            state.calculation_start = None;
            Task::none()
        }
        Message::ProgressUpdate => {
            // No longer used, but kept for compatibility
            Task::none()
        }
        Message::HashCalculated(algorithm, hash) => {
            state.hash_results.insert(algorithm, hash);
            Task::none()
        }
        Message::SettingsButtonPressed => {
            if !state.window_manager.any_of(&SettingsWindow) {
                let (_id, task) = state.window_manager.open(Box::new(SettingsWindow));
                task.map(|_id| {
                    Message::WindowClosed(iced::window::Id::unique())
                })
            } else {
                Task::none()
            }
        }
        Message::WindowClosed(id) => {
            // Check if the main window was closed BEFORE calling was_closed
            // (was_closed removes the window from the list)
            let main_instances = state.window_manager.instances_of(&MainWindow);
            let is_main_window = main_instances.iter().any(|(window_id, _)| **window_id == id);
            
            // Now mark the window as closed
            state.window_manager.was_closed(id);
            
            // If the main window was closed, exit the application immediately
            // (even if the settings window is still open)
            if is_main_window {
                process::exit(0);
            }
            
            Task::none()
        }
        Message::AlgorithmToggled(algorithm, enabled) => {
            if enabled {
                if !state.selected_algorithms.contains(&algorithm) {
                    state.selected_algorithms.push(algorithm);
                }
            } else {
                state.selected_algorithms.retain(|alg| *alg != algorithm);
            }
            
            // Save configuration when changed
            if let Err(e) = save_config(&state.selected_algorithms, state.hash_format, state.gost_variant, state.tiger_variant) {
                eprintln!("Error saving configuration: {}", e);
            }
            
            Task::none()
        }
        Message::FormatChanged(format) => {
            state.hash_format = format;
            
            // Reconvert all existing hashes to the new format
            let mut new_results = HashMap::new();
            for (algorithm, hash_str) in &state.hash_results {
                // Try to decode the current hash and convert it to the new format
                // First try as hex, then as base64
                use base64::Engine;
                if let Ok(bytes) = hex::decode(hash_str.trim().replace(' ', "")) {
                    new_results.insert(algorithm.clone(), format.format_hash(&bytes));
                } else if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(hash_str.trim().replace(' ', "")) {
                    new_results.insert(algorithm.clone(), format.format_hash(&bytes));
                } else {
                    // If it cannot be decoded, keep it as is
                    new_results.insert(algorithm.clone(), hash_str.clone());
                }
            }
            state.hash_results = new_results;
            
            // Save configuration
            if let Err(e) = save_config(&state.selected_algorithms, state.hash_format, state.gost_variant, state.tiger_variant) {
                eprintln!("Error saving configuration: {}", e);
            }
            
            Task::none()
        }
        Message::GostVariantChanged(variant) => {
            state.gost_variant = variant;
            // Changing variant invalidates existing GOST hashes; clear them
            state.hash_results.remove(&crate::hash::Algorithm::GOST);
            
            if let Err(e) = save_config(&state.selected_algorithms, state.hash_format, state.gost_variant, state.tiger_variant) {
                eprintln!("Error saving configuration: {}", e);
            }
            
            Task::none()
        }
        Message::TigerVariantChanged(variant) => {
            state.tiger_variant = variant;
            state.hash_results.remove(&crate::hash::Algorithm::TIGER192);
            
            if let Err(e) = save_config(&state.selected_algorithms, state.hash_format, state.gost_variant, state.tiger_variant) {
                eprintln!("Error saving configuration: {}", e);
            }
            
            Task::none()
        }
        Message::CopyHash(algorithm) => {
            if let Some(hash) = state.hash_results.get(&algorithm) {
                let hash_clone = hash.clone();
                Task::perform(
                    async move {
                        let mut clipboard = arboard::Clipboard::new().ok()?;
                        clipboard.set_text(hash_clone).ok()?;
                        Some(())
                    },
                    |_| Message::WindowClosed(iced::window::Id::unique()), // Dummy message, doesn't matter
                )
            } else {
                Task::none()
            }
        }
    }
}

pub fn view(state: &RealOne, window_id: iced::window::Id) -> Element<'_, Message, iced::Theme> {
    let settings_instances = state.window_manager.instances_of(&SettingsWindow);
    let is_settings_window = settings_instances.iter().any(|(id, _)| *id == &window_id);
    
    if is_settings_window {
        SettingsWindow.view(state)
    } else {
        let main_instances = state.window_manager.instances_of(&MainWindow);
        let is_main_window = main_instances.iter().any(|(id, _)| *id == &window_id);
        
        if is_main_window {
            views::view_main(state)
        } else {
            views::view_main(state)
        }
    }
}

pub fn title(state: &RealOne, window_id: iced::window::Id) -> String {
    let settings_instances = state.window_manager.instances_of(&SettingsWindow);
    let is_settings_window = settings_instances.iter().any(|(id, _)| *id == &window_id);
    
    if is_settings_window {
        SettingsWindow.title(state)
    } else {
        let main_instances = state.window_manager.instances_of(&MainWindow);
        let is_main_window = main_instances.iter().any(|(id, _)| *id == &window_id);
        
        if is_main_window {
            MainWindow.title(state)
        } else {
            "Real One - File Hash Checker".to_string()
        }
    }
}

pub fn subscription(_state: &RealOne) -> Subscription<Message> {
    iced::window::close_events().map(Message::WindowClosed)
}

