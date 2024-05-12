use crate::app::setup::MAIN;
use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts(["ctrl+\\"])?
            .with_handler(|app, shortcut, event| {
                if event.state == ShortcutState::Pressed
                    && shortcut.matches(Modifiers::CONTROL, Code::Backslash)
                {
                    let window = app.get_window(MAIN).unwrap();

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                    }
                }
            })
            .build(),
    )?;

    Ok(())
}
