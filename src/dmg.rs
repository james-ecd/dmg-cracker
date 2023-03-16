use cipher::KeyInit;
use random_string::generate;
use std::io::Write;
use std::process::{Command, Stdio};

const BASE_MOUNT_PATH: &str = "/tmp/";

pub struct Dmg {
    pub dmg_path: String,
    pub mount_path: String,
}

impl Dmg {
    pub fn new(dmg_path: &str) -> Self {
        Self {
            dmg_path: dmg_path.to_string(),
            mount_path: generate_random_mount_path(),
        }
    }

    pub fn attempt_password(&self, password: &str) -> bool {
        // if self.fast_verify_128(password) {
        //     self.full_verify(password)
        // } else {
        //     false
        // }
        self.fast_verify_128(password)
    }

    fn fast_verify_128(&self, password: &str) -> bool {}

    fn full_verify(&self, password: &str) -> bool {
        let mut child = Command::new("hdiutil")
            .arg("verify")
            .arg("-stdinpass")
            .arg(&self.dmg_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to execute hdiutil");

        child
            .stdin
            .as_mut()
            .expect("failed to open stdin")
            .write_all(password.as_bytes())
            .expect("failed to write password to stdin");

        let output =
            child.wait_with_output().expect("failed to wait on hdiutil");
        output.status.success()
    }
}

pub fn generate_random_mount_path() -> String {
    format!(
        "{}{}",
        BASE_MOUNT_PATH,
        generate(6, "abcdefghijklmnopqrstuvwxyz")
    )
}
