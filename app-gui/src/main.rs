// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(true, windows_subsystem = "windows")]

use std::error::Error;
use ui_render::render_ui;

pub mod ui_render;

fn main() -> Result<(), Box<dyn Error>> {
    render_ui()
}
