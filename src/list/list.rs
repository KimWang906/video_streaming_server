use std::{convert::Infallible};
use axum::{Json, response::{Response, IntoResponse}};
use hyper::{StatusCode, Body};
use serde::{Serialize, Deserialize};
use tokio::{fs, io};
use crate::video::video::VideoData;

const PATH: &str = "/src/resources";

#[derive(Serialize, Deserialize)]
struct VideoList<'list> {
    list: Vec<VideoData<'list>>
}

impl<'a> VideoList<'a> {
    fn new() -> Self {
        Self { 
            list: Vec::new()
        }
    }

    fn append<'video_method>(&mut self, season: &'video_method str, episode: &'video_method str) {
        self.list.push(VideoData::new(season, episode))
    }
}


pub async fn video_list_handler<'a>() -> Result<Json<VideoList<'a>>, io::Error> {
    let mut entries = fs::read_dir(PATH).await?;
    let mut video_list = VideoList::new();


    while let Ok(Some(entry)) = entries.next_entry().await {
        let f_name = entry.file_name();
        let f_name = f_name.to_str().unwrap();
        
        let data: Vec<&str> = f_name.split('-').collect();
        
        video_list.append(data[0], data[1]);

    }

    Ok(Json(video_list))
}

pub async fn handler<'a>() -> Result<impl IntoResponse, Infallible> {
    let get_video_list = video_list_handler().await;

    match get_video_list {
        Ok(res) => {
            let json_str = serde_json::to_string(&res.0).unwrap();
            let resp: Response<Body> = Response::new(json_str.into());
            Ok(resp.into_response())
        },
        Err(_) => {
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}