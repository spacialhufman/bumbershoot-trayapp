pub mod runners {
    use std::{ process::Command, collections::LinkedList, os::windows::process::CommandExt };
    use tauri::SystemTrayHandle;

    pub trait Runner {
        fn execute(&mut self, id: &String, tray_handle: &SystemTrayHandle);
    }

    pub struct AppRunner {
        app_name: String,
        php_version: String,
        app_folder: String,
        host: String,
        port: u32,
        pid: Option<u32>,
    }

    impl AppRunner {
        pub fn new(
            app_name: String,
            php_version: String,
            app_folder: String,
            host: String,
            port: u32
        ) -> AppRunner {
            AppRunner {
                app_name,
                php_version,
                app_folder,
                host,
                port,
                pid: None,
            }
        }

        fn start(&mut self) {
            let command = Command::new(format!("c:/wamp64/bin/php/php{}/php", self.php_version))
                .arg(&self.app_folder)
                .arg("serve")
                .arg(format!("--host={}", self.host))
                .arg(format!("--port={}", self.port))
                .creation_flags(0x08000000)
                .spawn()
                .expect("Não deu para rodar o MyGuest");

            self.pid = Some(command.id());
        }

        pub fn stop(&mut self) {
            if self.pid.is_none() {
                return;
            }

            Command::new("taskkill")
                .arg("/F")
                .arg("/T")
                .arg("/PID")
                .arg(self.pid.unwrap().to_string())
                .spawn()
                .expect("Failed to kill command");

            self.pid = None;
        }
    }

    impl Runner for AppRunner {
        fn execute(&mut self, id: &String, tray_handle: &SystemTrayHandle) {
            let item_handle = tray_handle.get_item(&id);

            if self.pid.is_none() {
                self.start();

                item_handle.set_selected(true).unwrap();
                item_handle.set_title(format!("Stop {}", self.app_name)).unwrap();
            } else {
                self.stop();

                item_handle.set_selected(false).unwrap();
                item_handle.set_title(format!("Serve {}", self.app_name)).unwrap();
            }
        }
    }

    pub struct SystemRunner<'a> {
        runners: LinkedList<&'a AppRunner>,
    }

    impl SystemRunner<'_> {
        pub fn new<'a>(runners: LinkedList<&'a AppRunner>) -> SystemRunner {
            SystemRunner { runners }
        }
    }

    impl Runner for SystemRunner<'_> {
        fn execute(&mut self, _id: &String, _tray_handle: &SystemTrayHandle) {
            for runner in &mut self.runners {
                runner.stop();
            }

            std::process::exit(0);
        }
    }
}