pub mod factories {
    use std::collections::HashMap;
    use crate::app_runner::app_runner::AppRunner;
    use crate::php_dispatcher::php::{PhpVersion, PhpDispatcher};

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
}
