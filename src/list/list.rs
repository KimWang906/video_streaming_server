use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub name: String,
    pub seasons: Vec<String>,
    episodes: HashMap<String, Vec<String>>,
}

impl Video {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            seasons: vec![],
            episodes: HashMap::new(),
        }
    }

    pub fn push_episode(&mut self, season: &str, episode: &str) {
        match self.episodes.get_mut(season) {
            Some(episodes) => episodes.push(episode.to_owned()),
            None => {
                self.seasons.push(season.to_owned());
                self.episodes
                    .insert(season.to_owned(), vec![episode.to_owned()]);
            }
        }
    }
}
