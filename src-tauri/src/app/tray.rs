use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    App, Error as TauriError, Manager,
};

pub fn init(app: &App) -> Result<(), TauriError> {
    let show = MenuItemBuilder::with_id("show", "Show")
        .accelerator("Cmd+S")
        .build(app)?;
    let hide = MenuItemBuilder::with_id("hide", "Hide")
        .accelerator("Cmd+H")
        .build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit")
        .accelerator("Cmd+Q")
        .build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&show, &hide, &quit])
        .build()?;
    TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(move |app, event| {
            let window = app.get_window("main").unwrap();

            match event.id().as_ref() {
                "show" => window.show().unwrap(),
                "hide" => window.hide().unwrap(),
                "quit" => std::process::exit(0),
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if event.click_type == ClickType::Left {
                let app = tray.app_handle();
                if let Some(webview_window) = app.get_window("main") {
                    let _ = webview_window.show();
                    let _ = webview_window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
