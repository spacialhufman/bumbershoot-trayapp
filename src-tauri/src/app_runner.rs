pub mod app_runner {
    use crate::php_dispatcher::php::PhpDispatcher;

    pub struct AppRunner {
        pub name: String,
        pub hash_name: String,
        pub php_dispatcher: PhpDispatcher,
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
}


