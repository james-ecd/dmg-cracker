use dmg_cracker;

use regex::Regex;

fn is_mounting_path_valid(generated_path: &str) -> bool {
    let matching_regex = Regex::new(r"/tmp/[a-z]{6}$").unwrap();
    matching_regex.is_match(&generated_path.to_string())
}

#[test]
fn generate_random_mount_path_returns_correctly() {
    let generated_path = dmg_cracker::dmg::generate_random_mount_path();
    assert!(is_mounting_path_valid(&generated_path));
}

#[test]
fn creating_new_dmg_struct_inits_correctly() {
    let dmg_path = "dmg_path";
    let dmg = dmg_cracker::dmg::Dmg::new(&dmg_path);

    assert_eq!(dmg_path, dmg.dmg_path);
    assert!(is_mounting_path_valid(&dmg.mount_path));
}

#[test]
fn attempt_password_calls_correct_funcs() {}
