use std::{ fs, io::{ Error, ErrorKind } };
use serde_json::Value;

pub fn json() -> Result<Value, Error> {
    let config = fs::File::open("config.json")?;

    let config_json: Value = match serde_json::from_reader(config) {
        Ok(config_json) => config_json,
        Err(_) => {
            return Err(
                Error::new(ErrorKind::Other, "Can't parse file to JSON")
            );
        }
    };

    Ok(config_json)
}
