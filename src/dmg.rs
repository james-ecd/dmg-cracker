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
            mount_path: generate_random_string(),
        }
    }

    pub fn attempt_password(&self, password: &str) -> bool {
        self.attempt_attach(password)
    }

    fn attempt_attach(&self, password: &str) -> bool {
        let mut child = match Command::new("hdiutil")
            .arg("verify")
            .arg("-stdinpass")
            .arg(&self.dmg_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                eprintln!("❌ Error: Failed to execute hdiutil command: {e}");
                eprintln!("💡 Please ensure:");
                eprintln!("   - You're running on macOS (hdiutil is required)");
                eprintln!("   - You have permission to access the DMG file");
                eprintln!("   - The DMG file exists and is not corrupted");
                return false;
            }
        };

        if let Some(stdin) = child.stdin.as_mut() {
            if let Err(e) = stdin.write_all(password.as_bytes()) {
                eprintln!("❌ Error: Failed to write password to hdiutil: {e}");
                return false;
            }
        } else {
            eprintln!("❌ Error: Failed to get stdin handle for hdiutil");
            return false;
        }

        match child.wait_with_output() {
            Ok(output) => output.status.success(),
            Err(e) => {
                eprintln!("❌ Error: Failed to wait for hdiutil process: {e}");
                false
            }
        }
    }
}

pub fn generate_random_string() -> String {
    format!(
        "{}{}",
        BASE_MOUNT_PATH,
        generate(6, "abcdefghijklmnopqrstuvwxyz")
    )
}
