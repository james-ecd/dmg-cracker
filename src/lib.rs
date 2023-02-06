use std::fs::File;
use std::io;
use std::io::Read;


pub fn read_password_list(filepath: &str) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n").map(String::from).collect::<Vec<_>>();
    Ok(lines)
}