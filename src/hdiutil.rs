use std::io::Write;
use std::process::{Command, Stdio};

pub fn attempt_password(dmg_path: &str, password: &str) -> bool {
    let mount_path = "/tmp/dmg_mount";
    let success = attempt_attach(&dmg_path, &password, &mount_path);
    if attempt_attach(&dmg_path, &password, &mount_path) {
        assert!(detach(&mount_path));
    }
    success
}

fn attempt_attach(dmg_path: &str, password: &str, mount_path: &str) -> bool {
    let mut child = Command::new("hdiutil")
        .arg("attach")
        .arg("-readonly")
        .arg("-mountpoint")
        .arg(&mount_path)
        .arg("-stdinpass")
        .arg(dmg_path)
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

fn detach(mount_path: &str) -> bool {
    let output = Command::new("hdiutil")
        .arg("detach")
        .arg(&mount_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("failed to execute hdiutil");
    output.status.success()
}