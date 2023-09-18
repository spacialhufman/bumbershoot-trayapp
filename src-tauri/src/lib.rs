pub mod lib {
    use std::{collections::HashMap, process::Command};
    use std::os::windows::process::CommandExt;

    fn factory_php72() -> PhpVersion {
        PhpVersion { path: "c:/wamp64/bin/php/php7.2.33/php".to_string() }
    }

    fn factory_php74() -> PhpVersion {
        PhpVersion { path: "c:/wamp64/bin/php/php7.4.9/php".to_string() }
    }

    pub fn factory_app_list() -> HashMap<String, AppRunner> {
        let mut apps: HashMap<String, AppRunner> = HashMap::new();

        // fetch applications stored in sql.lite
        // insert each one in the hashmap
        // keep going

        apps.insert("run-myguest".to_string(), AppRunner {
            name: "MyGuest".to_string(),
            hash_name: "run-myguest".to_string(),
            php_dispatcher: PhpDispatcher {
                pid: None,
                php: factory_php72(),
                host: "dev-painel.myguest.com.br".to_string(),
                app_folder: "c:/wamp64/www/wispot/myguest-new/artisan".to_string(),
                port: 8002
            }
        });

        apps.insert("run-wispot_api".to_string(), AppRunner {
            name: "Wispot API".to_string(),
            hash_name: "run-wispot_api".to_string(),
            php_dispatcher: PhpDispatcher {
                pid: None,
                php: factory_php74(),
                host: "dev-api.wispot.com.br".to_string(),
                app_folder: "c:/wamp64/www/wispot/api.wispot.com.br/artisan".to_string(),
                port: 8003
            }
        });

        apps.insert("run-wispot_integration".to_string(), AppRunner {
            name: "Wispot Integration".to_string(),
            hash_name: "run-wispot_integration".to_string(),
            php_dispatcher: PhpDispatcher {
                pid: None,
                php: factory_php74(),
                host: "dev-integracao.wispot.com.br".to_string(),
                app_folder: "c:/wamp64/www/wispot/integracao.wispot.com.br/artisan".to_string(),
                port: 8008
            }
        });

        apps
    }

    pub struct AppRunner {
        pub name: String,
        pub hash_name: String,
        php_dispatcher: PhpDispatcher,
    }

    pub struct PhpDispatcher {
        pid: Option<u32>,
        php: PhpVersion,
        app_folder: String,
        host: String,
        port: u32,
    }

    pub struct PhpVersion {
        path: String,
    }

    impl AppRunner {
        pub fn run_application(&mut self) {
            self.php_dispatcher.dispatch();
        }

        pub fn is_running(&self) -> bool {
            self.php_dispatcher.pid.is_some()
        }

        pub fn stop(&mut self) {
            self.php_dispatcher.kill();
        }
    }

    impl PhpDispatcher {
        pub fn dispatch(&mut self) {
            let command = Command::new(&self.php.path)
                .arg(&self.app_folder)
                .arg("serve")
                .arg(format!("--host={}", self.host))
                .arg(format!("--port={}", self.port))
                .creation_flags(0x08000000)
                .spawn()
                .expect("Não deu para rodar a aplicação");

            self.pid = Some(command.id());
        }

        pub fn kill(&mut self) {
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
}
