use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process::{Command, Stdio};


fn read_password_dict(filepath: &str) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n").map(String::from).collect::<Vec<_>>();
    Ok(lines)
}

fn attempt_password(dmg_path: &str, password: &str) -> bool {
    let mut child = Command::new("hdiutil")
        .arg("attach")
        .arg("-readonly")
        .arg("-mountpoint")
        .arg("/tmp/dmg_mount")
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
    let success = output.status.success();

    // Unmount the disk
    if success {
        let output = Command::new("hdiutil")
            .arg("detach")
            .arg("/tmp/dmg_mount")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .expect("failed to execute hdiutil");
        assert!(output.status.success());
    }

    success
}


fn main() {
    let passwords = read_password_dict("test.txt").unwrap();
    let mut found = false;
    for p in passwords {
        let success = attempt_password("test.dmg", &p);
        if success {
            println!("Password found: {}", &p);
            found = true;
            break;
        }
    }
    if !found {
        println!("No matching password found");
    }
}
