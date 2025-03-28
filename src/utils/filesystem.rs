use crate::errors;
use std::{fs, io::BufReader, path::Path};

pub fn load_data<T>(filename: String) -> Result<Vec<T>, errors::CliError>
where
    T: serde::de::DeserializeOwned,
{
    let path = Path::new(&filename);
    let file = fs::File::open(&path).map_err(|e| errors::CliError::IoError(e))?;

    let reader = BufReader::new(file);

    if path.extension().and_then(|e| e.to_str()) == Some("json") {
        let data: Vec<T> = serde_json::from_reader(reader)?;
        Ok(data)
    } else {
        Err(errors::CliError::UnsupportedFormat(filename))
    }
}
