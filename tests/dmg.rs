use dmg_cracker;

use regex::Regex;

#[test]
fn generate_random_mount_path_returns_correctly() {
    let matching_regex = Regex::new(r"/tmp/[a-z]{6}$").unwrap();
    let generated_path = dmg_cracker::dmg::generate_random_mount_path();
    assert!(matching_regex.is_match(&generated_path.to_string()));
}

#[test]
fn creating_new_dmg_struct_inits_correctly() {}

#[test]
fn attempt_password_calls_correct_funcs() {}
