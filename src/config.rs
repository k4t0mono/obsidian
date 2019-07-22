use serde::{Serialize, Deserialize};

const CFG_DIR: &'static str = "config/";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    daily: bool,
    reverse: bool,
    decks: Vec<String>,
}

impl Config {

    pub fn get_deck(&self, i: usize) -> &String {
        self.decks.get(i).unwrap()
    }

    pub fn loads() -> Config {
        let cfg = match Config::read() {
            Ok(c) => c,
            Err(_) => Config::create(),
        };

        cfg
    }

    fn create() -> Config {
        let c = Config {
            daily: true,
            reverse: false,
            decks: vec!(),
        };

        c.write().unwrap();

        c
    }

    fn read() -> std::io::Result<Config> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(format!("{}/cfg.toml", CFG_DIR))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = toml::from_str(&contents).unwrap();

        Ok(config)
    }

    fn write(&self) -> std::io::Result<()> {
        info!("Config file not found, creating a new one");
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::create(format!("{}/cfg.toml", CFG_DIR))?;

        let t = toml::to_string(&self).unwrap();
        file.write_all(t.as_bytes())?;

        Ok(())
    }
}