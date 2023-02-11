use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_password_list(filepath: &str) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split('\n').map(String::from).collect::<Vec<_>>();
    Ok(lines)
}

#[cfg(test)]
mod test_read_password_list {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_password_list() {
        let file_contents = "password1\npassword2\npassword3";
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(file_contents.as_bytes()).unwrap();

        let password_list =
            read_password_list(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(password_list, vec!["password1", "password2", "password3"]);
    }
}
