pub mod myguest {
    use std::{ process::Command };
    use std::os::windows::process::CommandExt;

    pub struct MyGuest {
        pub pid: Option<u32>,
    }

    impl MyGuest {
        pub fn new() -> MyGuest {
            MyGuest {
                pid: None,
            }
        }

        pub fn start(&mut self) {
            let command = Command::new("c:/wamp64/bin/php/php7.2.33/php")
                .arg("c:/wamp64/www/wispot/myguest-new/artisan")
                .arg("serve")
                .arg("--host=dev-painel.myguest.com.br")
                .arg("--port=8002")
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