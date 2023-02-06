mod hdiutil;
mod lib;

use crate::hdiutil::attempt_password;
use crate::lib::read_password_list;

use indicatif::ProgressBar;

fn main() {
    let passwords = read_password_list("test.txt").unwrap();
    let pb = ProgressBar::new(passwords.len().try_into().unwrap());
    let mut found = false;
    for p in passwords {
        let success = attempt_password("test.dmg", &p);
        pb.inc(1);
        if success {
            println!("Password found: {}", &p);
            pb.finish_with_message(format!("Password found: {}", &p));
            pb.finish_and_clear();
            found = true;
            break;
        }
    }
    if !found {
        println!("No matching password found");
    }
}
