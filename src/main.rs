fn main() {
    if let Err(e) = dmg_cracker::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
