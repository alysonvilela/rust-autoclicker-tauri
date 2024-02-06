// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
use std::sync::{Arc, Mutex};

use enigo::*;
use tauri::*;

use rdev::{listen, EventType};
use winit::{
    event::{self, Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn send(event_type: &EventType) {
    let delay = std::time::Duration::from_millis(20);
    match rdev::simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    std::thread::sleep(delay);
}


#[tauri::command]
fn start_afk(start_pos: (i32, i32), end_pos: (i32, i32)) {
    println!("I was invoked from JS! {:?} {:?}", start_pos, end_pos);
}

#[tauri::command]
fn get_next_click(status: String, app: tauri::AppHandle) -> bool {
    watch_mouse(status, app);
    return true;
}

#[tauri::command]
fn watch_mouse(status: String, app_handle: tauri::AppHandle) {
    println!("Command received: {}", status);
    Command::new("open")
        .arg("-a")
        .arg("Slack")
        .spawn()
        .expect("Failed to spawn process");

    let mut pos: Option<(i32, i32)> = None; 
    if let Err(error) = listen(move |event| {
        callback(event, &app_handle, &mut pos, &status);
        // If click event occurred, return Err to stop listening
        let _ = pos.is_none();
    }) {
    }
}

fn callback(event: rdev::Event, app_handle: &tauri::AppHandle, pos: &mut Option<(i32, i32)>, status: &String) {
    if pos.is_some() {
        return;
    }
    
    match event.event_type {
        EventType::ButtonPress(button) => match button {
            rdev::Button::Left => {
                let mut enigo_instance = Enigo::new();
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
                send(&EventType::KeyPress(rdev::Key::MetaLeft));
                send(&EventType::KeyPress(rdev::Key::MetaLeft));
                send(&EventType::KeyPress(rdev::Key::MetaLeft));
                send(&EventType::KeyPress(rdev::Key::MetaLeft));
                send(&EventType::KeyPress(rdev::Key::KeyM));

                println!("Clicked {},{}", x, y);
            }
            _ => {}
        },
        EventType::ButtonRelease(_) => {
        }
        EventType::KeyPress(_) => {
        }
        EventType::KeyRelease(_) => {
        }
        _ => {
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_next_click, start_afk])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
