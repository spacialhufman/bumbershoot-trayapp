pub mod application {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use tauri::{
        Builder,
        SystemTrayMenu,
        CustomMenuItem,
        SystemTrayMenuItem,
        SystemTray,
    };

    use crate::runners::runners::Runner;
    use crate::event_handler::event_handlers::TauriEventHandler;

    pub struct BumbershootApplication<'a> {
        app_runner: Arc<Mutex<HashMap<String, &'a (dyn Runner + Sync + Send)>>>,
    }

    impl BumbershootApplication<'_> {
        pub fn new<'a>(app_runner: HashMap<String, &'a (dyn Runner + Sync + Send)>) -> BumbershootApplication {
            BumbershootApplication {
                app_runner: Arc::new(Mutex::new(app_runner))
            }
        }

        pub fn run(&self) {
            let tray_menu = SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("run-myguest".to_string(), "Serve MyGuest"))
                .add_item(CustomMenuItem::new("run-sending".to_string(), "Serve Sending"))
                .add_item(CustomMenuItem::new("run-wispot_api".to_string(), "Serve Wispot API"))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(CustomMenuItem::new("close".to_string(), "Sair"));

            let app_runner = self.app_runner.clone();

            Builder::default()
                .system_tray(SystemTray::new().with_menu(tray_menu))
                .on_system_tray_event(
                    move |app, event| TauriEventHandler::on_tray_event_handler(&app, event, &app_runner)
                )
                .on_window_event(|event| TauriEventHandler::on_window_event_handler(event))
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
    }
}