#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use kalk::parser::{self, Context};
use tauri::command;

pub struct AbacusState {
    kalk_context: Context,
}

type ProtectedAbacusState = Mutex<AbacusState>;

impl Default for AbacusState {
    fn default() -> Self {
        Self {
            kalk_context: Context::new(),
        }
    }
}

#[command]
fn eval(expression: String, state: tauri::State<ProtectedAbacusState>) -> Result<String, String> {
    let context = &mut state.lock().unwrap().kalk_context;
    let result = parser::eval(context, &expression, 55);

    match result {
        Ok(r) => Ok(match r {
            Some(num) => num.to_string_pretty(),
            None => String::default(),
        }),
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(ProtectedAbacusState::default())
        .invoke_handler(tauri::generate_handler![eval])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
