pub mod application {
    use std::sync::{ Arc, Mutex };
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

    use crate::myguest::myguest::MyGuest;
    use crate::sending::sending::Sending;
    use crate::wispot_api::wispot_api::WispotApi;

    pub struct BumbershootApp {
        myguest: Arc<Mutex<MyGuest>>,
        sending: Arc<Mutex<Sending>>,
        wispot_api: Arc<Mutex<WispotApi>>,
    }

    impl BumbershootApp {
        pub fn new(myguest: MyGuest, sending: Sending, wispot_api: WispotApi) -> BumbershootApp {
            BumbershootApp {
                myguest: Arc::new(Mutex::new(myguest)),
                sending: Arc::new(Mutex::new(sending)),
                wispot_api: Arc::new(Mutex::new(wispot_api)),
            }
        }

        pub fn run(&self) {
            let tray_menu = SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("run-myguest".to_string(), "Serve MyGuest"))
                .add_item(CustomMenuItem::new("run-sending".to_string(), "Serve Sending"))
                .add_item(CustomMenuItem::new("run-wispot_api".to_string(), "Serve Wispot API"))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(CustomMenuItem::new("close".to_string(), "Sair"));

            let myguest = self.myguest.clone();
            let sending = self.sending.clone();
            let wispot_api = self.wispot_api.clone();

            Builder::default()
                .system_tray(SystemTray::new().with_menu(tray_menu))
                .on_system_tray_event(
                    move |app, event| Self::on_tray_event_handler(
                        app, event, &myguest, &sending, &wispot_api
                    )
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
            app: &AppHandle,
            event: SystemTrayEvent,
            myguest: &Arc<Mutex<MyGuest>>,
            sending: &Arc<Mutex<Sending>>,
            wispot_api: &Arc<Mutex<WispotApi>>
        ) {
            match event {
                SystemTrayEvent::DoubleClick { tray_id: _, position: _, size: _, .. } => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    let item_handle = app.tray_handle().get_item(&id);

                    match id.as_str() {
                        "run-myguest" => {
                            if myguest.lock().unwrap().pid.is_none() {
                                myguest.lock().unwrap().start();

                                item_handle.set_selected(true).unwrap();
                                item_handle.set_title("Stop MyGuest").unwrap();
                            } else {
                                myguest.lock().unwrap().stop();

                                item_handle.set_selected(false).unwrap();
                                item_handle.set_title("Serve MyGuest").unwrap();
                            }
                        }
                        "run-sending" => {
                            if sending.lock().unwrap().pid.is_none() {
                                sending.lock().unwrap().start();

                                item_handle.set_selected(true).unwrap();
                                item_handle.set_title("Stop Sending").unwrap();
                            } else {
                                sending.lock().unwrap().stop();

                                item_handle.set_selected(false).unwrap();
                                item_handle.set_title("Serve Sending").unwrap();
                            }
                        }
                        "run-wispot_api" => {
                            if wispot_api.lock().unwrap().pid.is_none() {
                                wispot_api.lock().unwrap().start();

                                item_handle.set_selected(true).unwrap();
                                item_handle.set_title("Stop Wispot API").unwrap();
                            } else {
                                wispot_api.lock().unwrap().stop();

                                item_handle.set_selected(false).unwrap();
                                item_handle.set_title("Serve Wispot API").unwrap();
                            }
                        }
                        "close" => {
                            if myguest.lock().unwrap().pid.is_some() {
                                myguest.lock().unwrap().stop();
                            }

                            if sending.lock().unwrap().pid.is_some() {
                                sending.lock().unwrap().stop();
                            }

                            if wispot_api.lock().unwrap().pid.is_some() {
                                wispot_api.lock().unwrap().stop();
                            }

                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}