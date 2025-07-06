use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_password_list(
    filepath: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match extension.to_lowercase().as_str() {
        "csv" => read_csv_passwords(filepath),
        _ => read_txt_passwords(filepath),
    }
}

fn validate_password(password: &str) -> Option<String> {
    let trimmed = password.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn read_txt_passwords(
    filepath: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let passwords: Vec<String> =
        contents.lines().filter_map(validate_password).collect();
    Ok(passwords)
}

fn read_csv_passwords(
    filepath: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut passwords = Vec::new();

    for result in rdr.records() {
        let record = result?;
        // Take the first column as the password
        if let Some(password) = record.get(0) {
            if let Some(validated) = validate_password(password) {
                passwords.push(validated);
            }
        }
    }
    Ok(passwords)
}

#[cfg(test)]
mod test_read_password_list {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_password_list_txt() {
        let file_contents = "password1\npassword2\npassword3";
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(file_contents.as_bytes()).unwrap();

        let password_list =
            read_password_list(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(password_list, vec!["password1", "password2", "password3"]);
    }

    #[test]
    fn test_read_password_list_csv() {
        let file_contents = "password1\npassword2\npassword3\n";
        let mut temp_file = NamedTempFile::with_suffix(".csv").unwrap();
        temp_file.write_all(file_contents.as_bytes()).unwrap();

        let password_list =
            read_password_list(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(password_list, vec!["password1", "password2", "password3"]);
    }

    #[test]
    fn test_read_password_list_csv_with_headers() {
        let file_contents = "password,description\npassword1,common\npassword2,weak\npassword3,medium\n";
        let mut temp_file = NamedTempFile::with_suffix(".csv").unwrap();
        temp_file.write_all(file_contents.as_bytes()).unwrap();

        let password_list =
            read_password_list(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(
            password_list,
            vec!["password", "password1", "password2", "password3"]
        );
    }

    #[test]
    fn test_read_password_list_csv_with_quotes() {
        let file_contents = "\"password1\",\"description1\"\n\"password,2\",\"description2\"\n\"password3\",\"description3\"\n";
        let mut temp_file = NamedTempFile::with_suffix(".csv").unwrap();
        temp_file.write_all(file_contents.as_bytes()).unwrap();

        let password_list =
            read_password_list(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(password_list, vec!["password1", "password,2", "password3"]);
    }
}
