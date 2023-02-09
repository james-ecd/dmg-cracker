mod cli;
pub mod dmg;
pub mod passwords;

use crate::dmg::Dmg;
use crate::passwords::read_password_list;
use clap::Parser;
use indicatif::ProgressBar;

pub fn run() {
    let args = cli::Args::parse();
    let passwords = read_password_list(&args.password_list_path).unwrap();
    let dmg = Dmg::new(&args.dmg_path);
    let pb = ProgressBar::new(passwords.len().try_into().unwrap());
    for p in passwords {
        if dmg.attempt_password(&p) {
            println!("Password found: {}", &p);
            return;
        }
        pb.inc(1);
    }
    println!("No matching password found");
}
