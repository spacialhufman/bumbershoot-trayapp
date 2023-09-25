pub mod php {
    use std::{os::windows::process::CommandExt, process::Command};

    pub struct PhpDispatcher {
        pub pid: Option<u32>,
        pub php: PhpVersion,
        pub app_folder: String,
        pub host: String,
        pub port: u32,
    }

    pub struct PhpVersion {
        pub path: String,
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
