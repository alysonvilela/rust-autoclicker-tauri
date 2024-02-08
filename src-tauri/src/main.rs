// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use enigo::*;
use rdev::*;
use serde::Deserialize;
use tauri::*;
use tauri_plugin_positioner::*;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

use mouce::Mouse;
use std::thread;
use std::time::Duration;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Payload {
    startPos: (i32, i32),
    endPos: (i32, i32),
}

fn _move_mouse(start_pos: (i32, i32), end_pos: (i32, i32)) {
    let mut enigo_instance = Enigo::new();
    let five_secs = core::time::Duration::from_secs(5);
    loop {
        enigo_instance.mouse_move_to(start_pos.0, start_pos.1);
        enigo_instance.mouse_click(enigo::MouseButton::Left);
        std::thread::sleep(five_secs);

        enigo_instance.mouse_move_to(end_pos.0, end_pos.1);
        enigo_instance.mouse_click(enigo::MouseButton::Left);
        std::thread::sleep(five_secs);
    }
}

#[tauri::command]
fn get_next_click(status: String, app: tauri::AppHandle) -> bool {
    watch_mouse(status, app);
    return true;
}

#[tauri::command]
fn watch_mouse(status: String, app_handle: tauri::AppHandle) {
    println!("Command received: {}", status);

    let mut pos: Option<(i32, i32)> = None;
    if let Err(_error) = listen(move |event| {
        callback(event, &app_handle, &mut pos, &status);
        let _ = pos.is_none();
    }) {}
}

fn callback(
    event: rdev::Event,
    app_handle: &tauri::AppHandle,
    pos: &mut Option<(i32, i32)>,
    status: &String,
) {
    if pos.is_some() {
        return;
    }

    match event.event_type {
        EventType::ButtonPress(button) => match button {
            rdev::Button::Left => {
                let enigo_instance = Enigo::new();
                let (x, y) = enigo_instance.mouse_location();
                *pos = Some((x, y));
                if status == "start" {
                    app_handle
                        .emit_all("start_position_at", (x, y))
                        .expect("Failed to emit event");
                }
                if status == "end" {
                    app_handle
                        .emit_all("end_position_at", (x, y))
                        .expect("Failed to emit event");
                }

                app_handle.show().expect("err on open");

                println!("Clicked {},{}", x, y);
            }
            _ => {}
        },
        EventType::ButtonRelease(_) => {}
        EventType::KeyPress(_) => {}
        EventType::KeyRelease(_) => {}
        _ => {}
    }
}

fn main() {
    let quit = tauri::CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray = tauri::SystemTrayMenu::new().add_item(quit);

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            window.listen_global("run-move-mouse", |event| {
                let payload = event.payload().unwrap();
                println!("position on win: {:?}", payload);
            });

            app.listen_global("run-move-mouse", |event| {
                let payload = event.payload().unwrap();
                println!("position on app: {:?}", payload);

                let data: Payload = serde_json::from_str(payload).expect("Algum erro ai");
                let end_pos: (i32, i32) = data.endPos;
                let start_pos: (i32, i32) = data.startPos;

                let mouse_manager = Mouse::new();

                // for _ in 0..10 {
                loop {
                    // Move to start_pos
                    let current_position = mouse_manager
                        .get_position()
                        .expect("error on find current position");
                    let steps_x_start = (start_pos.0 - current_position.0) as f64;
                    let steps_y_start = (start_pos.1 - current_position.1) as f64;
                    let total_steps_start = steps_x_start.max(steps_y_start) as usize;

                    for step in 1..=total_steps_start {
                        let new_x = current_position.0
                            + ((steps_x_start / total_steps_start as f64) * step as f64) as i32;
                        let new_y = current_position.1
                            + ((steps_y_start / total_steps_start as f64) * step as f64) as i32;

                        let _ = mouse_manager.move_to(new_x as usize, new_y as usize);
                        thread::sleep(Duration::from_millis(2));
                    }
                    mouse_manager
                        .click_button(&mouce::common::MouseButton::Left)
                        .expect("error on click");
                    // Wait 10 seconds before moving to end_pos
                    thread::sleep(Duration::from_secs(2));

                    // Move to end_pos
                    let steps_x_end = (end_pos.0 - current_position.0) as f64;
                    let steps_y_end = (end_pos.1 - current_position.1) as f64;
                    let total_steps_end = steps_x_end.max(steps_y_end) as usize;

                    for step in 1..=total_steps_end {
                        let new_x = current_position.0
                            + ((steps_x_end / total_steps_end as f64) * step as f64) as i32;
                        let new_y = current_position.1
                            + ((steps_y_end / total_steps_end as f64) * step as f64) as i32;

                        let _ = mouse_manager.move_to(new_x as usize, new_y as usize);
                        thread::sleep(Duration::from_millis(1));
                    }
                    mouse_manager
                        .click_button(&mouce::common::MouseButton::Left)
                        .expect("error on click");

                    // Wait 10 seconds before next iteration
                    thread::sleep(Duration::from_secs(2));
                }
            });

            #[cfg(target_os = "macos")]
            apply_vibrancy(
                &window,
                NSVisualEffectMaterial::Menu,
                Some(NSVisualEffectState::Active),
                Some(6.0),
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_next_click])
        .system_tray(
            tauri::SystemTray::new()
                .with_menu(system_tray)
                .with_title("Auto clicker"),
        )
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);

            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(tauri_plugin_positioner::Position::TrayLeft);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
