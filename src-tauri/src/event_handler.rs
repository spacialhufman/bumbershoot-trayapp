pub mod event_handlers {
    use std::{collections::HashMap, sync::{Arc, Mutex}};

    use tauri::{
        AppHandle,
        SystemTrayEvent,
        GlobalWindowEvent,
        WindowEvent,
        Manager
    };

    use crate::runners::runners::Runner;

    pub struct TauriEventHandler {}

    impl TauriEventHandler {
        pub fn on_tray_event_handler<'a>(
            app: &AppHandle,
            event: SystemTrayEvent,
            app_runner: &Arc<Mutex<HashMap<String, &'a (dyn Runner + Sync + Send)>>>
        ) {
            match event {
                SystemTrayEvent::DoubleClick { tray_id: _, position: _, size: _, .. } => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    let tray_handle = app.tray_handle();
                    let ref_app_runner = app_runner.lock().unwrap();

                    let mut _runner = *ref_app_runner.get(&id).unwrap();

                    _runner.execute(&id, &tray_handle);
                }
                _ => {}
            }
        }

        pub fn on_window_event_handler(event: GlobalWindowEvent) {
            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        }
    }
}