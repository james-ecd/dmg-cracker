use regex::Regex;

fn is_mounting_path_valid(generated_path: &str) -> bool {
    let matching_regex = Regex::new(r"/tmp/[a-z]{6}$").unwrap();
    matching_regex.is_match(generated_path)
}

#[test]
fn generate_random_string_returns_correctly() {
    let generated_path = dmg_cracker::dmg::generate_random_string();
    assert!(is_mounting_path_valid(&generated_path));
}

#[test]
fn creating_new_dmg_struct_inits_correctly() {
    let dmg_path = "dmg_path";
    let dmg = dmg_cracker::dmg::Dmg::new(dmg_path);

    assert_eq!(dmg_path, dmg.dmg_path);
    assert!(is_mounting_path_valid(&dmg.mount_path));
}
