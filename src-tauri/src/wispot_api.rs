pub mod wispot_api {
    use std::{ process::Command };
    use std::os::windows::process::CommandExt;

    pub struct WispotApi {
        pub pid: Option<u32>,
    }

    impl WispotApi {
        pub fn new() -> WispotApi {
            WispotApi { pid: None }
        }

        pub fn start(&mut self) {
            let command = Command::new("c:/wamp64/bin/php/php7.4.9/php")
                .arg("c:/wamp64/www/wispot/api.wispot.com.br/artisan")
                .arg("serve")
                .arg("--host=dev-api.wispot.com.br")
                .arg("--port=8003")
                .creation_flags(0x08000000)
                .spawn()
                .expect("Não deu para rodar o MyGuest");

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