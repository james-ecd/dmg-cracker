mod lib;
mod hdiutil;

use crate::hdiutil::attempt_password;
use crate::lib::read_password_list;


fn main() {
    let passwords = read_password_list("test.txt").unwrap();
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
