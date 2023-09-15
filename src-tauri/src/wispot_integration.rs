pub mod wispot_integration {
    use std::{ process::Command };
    use std::os::windows::process::CommandExt;

    pub struct WispotIntegration {
        pub pid: Option<u32>,
    }

    impl WispotIntegration {
        pub fn new() -> WispotIntegration {
            WispotIntegration { pid: None }
        }

        pub fn start(&mut self) {
            let command = Command::new("c:/wamp64/bin/php/php7.4.9/php")
                .arg("c:/wamp64/www/wispot/integracao.wispot.com.br/artisan")
                .arg("serve")
                .arg("--host=dev-integracao.wispot.com.br")
                .arg("--port=8008")
                .creation_flags(0x08000000)
                .spawn()
                .expect("Não deu para rodar o Wispot Integration");

            self.pid = Some(command.id());
        }

        pub fn stop(&mut self) {
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
