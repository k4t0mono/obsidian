#![allow(dead_code)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    test: String,
}

impl Config {

    pub fn new(test: String) -> Config {
        Config { test }
    }

    pub fn loads() -> Config {
        Config::read().unwrap_or(Config::create())
    }

    fn create() -> Config {
        let c = Config {
            test: "auto hewo".to_string()
        };

        c.write().unwrap();

        c
    }

    fn read() -> std::io::Result<Config> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open("cfg.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = toml::from_str(&contents).unwrap();

        Ok(config)
    }

    fn write(&self) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::create("cfg.toml")?;

        let t = toml::to_string(&self).unwrap();
        file.write_all(t.as_bytes())?;

        Ok(())
    }
}