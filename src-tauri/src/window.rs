use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager
};

pub fn create_system_tray(app: &tauri::App) {
    // 展示窗口
    if let (Ok(show_i), Ok(quit_i)) = (
        MenuItem::with_id(app, "show", "Show", true, None::<&str>),
        MenuItem::with_id(app, "quit", "Quit", true, None::<&str>),
    ) {
        if let Ok(menu) = Menu::with_items(app, &[&show_i, &quit_i]) {
            let _tray = TrayIconBuilder::new()
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    },
                    _ => {}
                })
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .build(app);
            println!("tray created");
        }
    }
}
