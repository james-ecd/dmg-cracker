mod dmg;
mod passwords;

use crate::dmg::Dmg;
use crate::passwords::read_password_list;
use clap::Parser;
use indicatif::ProgressBar;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to password list .txt file (passwords seperated by newlines)
    #[arg(short, long)]
    password_list_path: String,

    /// Path to encrypted .dmg file
    #[arg(short, long)]
    dmg_path: String,
}

fn main() {
    let args = Args::parse();
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
