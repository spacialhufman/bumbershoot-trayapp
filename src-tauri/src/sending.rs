pub mod sending {
    use std::{ process::Command };
    use std::os::windows::process::CommandExt;

    pub struct Sending {
        pub pid: Option<u32>,
    }

    impl Sending {
        pub fn new() -> Sending {
            Sending { pid: None }
        }

        pub fn start(&mut self) {
            let command = Command::new("c:/wamp64/bin/php/php7.4.9/php")
                .arg("c:/wamp64/www/wispot/sending/artisan")
                .arg("serve")
                .arg("--host=dev-painel.sending.com.br")
                .arg("--port=8082")
                .creation_flags(0x08000000)
                .spawn()
                .expect("Não deu para rodar o Sending");

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