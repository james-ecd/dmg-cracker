use random_string::generate;
use std::io::Write;
use std::process::{Command, Stdio};

const BASE_MOUNT_PATH: &str = "/tmp/";

pub struct Dmg {
    dmg_path: String,
    mount_path: String,
}

impl Dmg {
    pub fn new(dmg_path: &str) -> Self {
        Self {
            dmg_path: dmg_path.to_string(),
            mount_path: generate_random_mount_path(),
        }
    }

    pub fn attempt_password(&self, password: &str) -> bool {
        if self.attempt_attach(&password) {
            assert!(self.detach());
            return true;
        }
        false
    }

    fn attempt_attach(&self, password: &str) -> bool {
        let mut child = Command::new("hdiutil")
            .arg("attach")
            .arg("-readonly")
            .arg("-mountpoint")
            .arg(&self.mount_path)
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

        let output = child.wait_with_output().expect("failed to wait on hdiutil");
        output.status.success()
    }

    fn detach(&self) -> bool {
        let output = Command::new("hdiutil")
            .arg("detach")
            .arg(&self.mount_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .expect("failed to execute hdiutil");
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

#[cfg(test)]
mod test_dmg {
    use super::Dmg;

    #[test]
    fn attempt_attach_makes_correct_system_call() {}

    #[test]
    fn attempt_detach_makes_correct_system_call() {}
}
