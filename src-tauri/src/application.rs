pub mod application {
    use std::{collections::HashMap, sync::{Mutex, Arc}};
    use tauri::{
        Builder,
        Manager,
        SystemTrayMenu,
        CustomMenuItem,
        SystemTrayMenuItem,
        SystemTray,
        SystemTrayEvent,
        WindowEvent, AppHandle
    };

    use crate::app_runner::app_runner::AppRunner;

    pub struct BumbershootApp {
        app_list: Arc<Mutex<HashMap<String, AppRunner>>>,
    }

    impl BumbershootApp {
        pub fn new(app_list: HashMap<String, AppRunner>) -> BumbershootApp {
            BumbershootApp { app_list: Arc::new(Mutex::new(app_list)) }
        }

        pub fn run(self) {
            let mut tray_menu = SystemTrayMenu::new();

            let app_list = self.app_list.clone();

            {
                let hash_apps = &app_list.lock().unwrap();
                for key in hash_apps.keys() {
                    let app_runner = &hash_apps[key];

                    tray_menu = tray_menu.add_item(
                        CustomMenuItem::new(
                            app_runner.hash_name.clone(),
                            format!("Serve {}", app_runner.name.clone())
                        )
                    );
                }
            }

            tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
            tray_menu = tray_menu.add_item(CustomMenuItem::new("close".to_string(), "Sair"));

            Builder::default()
                .system_tray(SystemTray::new().with_menu(tray_menu))
                .on_system_tray_event(
                    move |app, event| Self::on_tray_event_handler(&app_list, app, event)
                )
                .on_window_event(|event| match event.event() {
                    WindowEvent::CloseRequested { api, .. } => {
                        event.window().hide().unwrap();
                        api.prevent_close();
                    }
                    _ => {}
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }

        fn on_tray_event_handler(
            app_list: &Arc<Mutex<HashMap<String, AppRunner>>>,
            app: &AppHandle,
            event: SystemTrayEvent
        ) {
            match event {
                SystemTrayEvent::DoubleClick { tray_id: _, position: _, size: _, .. } => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    let item_handle = app.tray_handle().get_item(&id);

                    match id.as_str() {
                        "close" => {
                            for (_, app_runner) in app_list.lock().unwrap().iter_mut() {
                                if app_runner.is_running() {
                                    app_runner.stop();
                                }
                            }

                            std::process::exit(0);
                        }
                        _ => {
                            let mut app_list_mut = app_list.lock().unwrap();
                            let app_runner       = app_list_mut.get_mut(id.as_str()).unwrap();

                            if !app_runner.is_running() {
                                app_runner.run_application();

                                item_handle.set_selected(true).unwrap();
                                item_handle.set_title(format!("Stop {}", &app_runner.name)).unwrap();
                            } else {
                                app_runner.stop();

                                item_handle.set_selected(false).unwrap();
                                item_handle.set_title(format!("Serve {}", &app_runner.name)).unwrap();
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
