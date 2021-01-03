use core::panic;
use serde::Deserialize;
use std::{
    fs,
    io::{BufReader, Read},
};
use toml;

#[derive(Debug, Deserialize)]
struct ApiKey {
    apikey: Option<Key>,
}
#[derive(Debug, Deserialize)]
struct Key {
    key_value: Option<String>,
}

fn read_config() -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open("ApiKey.toml")
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn get_config() -> String {
    let p = read_config();

    let s = match p {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    // println!("{}", s);

    let conf: Result<ApiKey, toml::de::Error> = toml::from_str(&s);

    // println!("----{:#?}", conf);

    let key = match conf {
        Ok(key) => key.apikey,
        Err(e) => panic!("conifg error: {}", e),
    };

    // println!("{:#?}", key);
    let value = match key {
        Some(value) => value,
        None => panic!("no conifg setted."),
    };

    let return_value = match value.key_value {
        Some(return_value) => return_value,
        None => panic!("no value"),
    };

    return_value

    // println!("{}",s);
}
