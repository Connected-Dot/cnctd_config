use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Error, self};
use std::path::Path;

pub struct Config<T> where T: Serialize + for<'de> Deserialize<'de> {
    pub data: T,
    pub file_path: String,
}

impl<T> Config<T> where T: Serialize + for<'de> Deserialize<'de> {
    pub fn new(data: T, file_path: Option<&str>) -> Self {
        Config {
            data,
            file_path: if file_path.is_some() { file_path.unwrap().to_string() } else { "./config.json".to_string() },
        }
    }

    pub fn update(&mut self, new_data: T) -> Result<(), Error> {
        self.data = new_data;
        self.save()
    }

    pub fn get(&self) -> &T {
        &self.data
    }

    pub fn save(&self) -> Result<(), Error> {
        let mut file = File::create(&self.file_path)?;
        let data_as_json = serde_json::to_string(&self.data)?;
        file.write_all(data_as_json.as_bytes())?;
        Ok(())
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut file = File::open(&self.file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        self.data = serde_json::from_str(&data)?;
        Ok(())
    }
}

