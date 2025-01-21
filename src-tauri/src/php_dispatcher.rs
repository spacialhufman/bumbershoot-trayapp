pub mod php {
    use std::process::{Command, Stdio};

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
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to run the application");

            self.pid = Some(command.id());
        }

        pub fn kill(&mut self) {
            if let Some(pid) = self.pid {
                Command::new("kill")
                    .arg("-9")
                    .arg(pid.to_string())
                    .spawn()
                    .expect("Failed to kill command");

                self.pid = None;
            }
        }
    }
}
