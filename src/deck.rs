use serde::{Serialize, Deserialize};

const CFG_DIR: &'static str = "config/";

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    name: String,
    about: String,
    keywords: Vec<String>,
    reverse: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    name: String,
    about: String,
    cards: Vec<Card>,
}

impl Deck {

    pub fn loads(name: &String) -> Deck {
        Deck::read(name).unwrap()
    }

    fn read(name: &String) -> std::io::Result<Deck> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(format!("{}/{}.toml", CFG_DIR, name))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let deck: Deck = toml::from_str(&contents).unwrap();

        Ok(deck)
    }

}