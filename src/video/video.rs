use std::{
    fmt::Display, 
};


#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoData {
    pub season: String,
    pub episode: String,
}

impl Display for VideoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl VideoData {
    pub fn new<'a>(season: &'a str, episode: &'a str) -> Self {
        Self {
            season: season.to_owned(), 
            episode: episode.to_owned()

        }
    }
}