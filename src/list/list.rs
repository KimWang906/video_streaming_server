use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VideoList {
    pub seasons: Vec<String>,
    sp: Vec<String>,
    s1: Vec<String>,
    s2: Vec<String>,
    s3: Vec<String>,
    s4: Vec<String>,
    s5: Vec<String>,
    s6: Vec<String>,
}

impl VideoList {
    pub fn new() -> Self {
        Self { 
            seasons: vec![],
            sp: vec![],
            s1: vec![],
            s2: vec![],
            s3: vec![],
            s4: vec![],
            s5: vec![],
            s6: vec![] 
        }
    }

    pub fn push_episode(&mut self, season: &str, episode: &str) {
        match season {
            "S1" => self.s1.push(episode.to_owned()),
            "S2" => self.s2.push(episode.to_owned()),
            "S3" => self.s3.push(episode.to_owned()),
            "S4" => self.s4.push(episode.to_owned()),
            "S5" => self.s5.push(episode.to_owned()),
            "S6" => self.s6.push(episode.to_owned()),
            "Sp" => self.sp.push(episode.to_owned()),
            _ => panic!("Season Not Found"),
        }
    }
}