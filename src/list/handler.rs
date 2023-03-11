use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::{Body, StatusCode};
use std::convert::Infallible;
use tokio::{fs, io};
use std::collections::HashMap;

use super::list::VideoList;

// video file의 path
pub const PATH: &str = "src/resources/my_hero_academia";

pub async fn video_list_handler() -> Result<Json<HashMap<String,String>>, io::Error> {
    // 1. Path의 Directory를 읽어들인다.
    let mut entries = fs::read_dir(PATH).await?;
    // 2. 최종적으로 반환할 VideoList를 생성한다.
    let mut list = VideoList::new();
    let mut animeList: HashMap<String, String> = HashMap::new();

    // 3. 디렉터리 내에 있는 시즌 별 디렉터리를 하나씩(dir_entry) 읽어들인다.
    while let Ok(Some(dir_entry)) = entries.next_entry().await {
        // 4. 변수 셰도잉으로 directory의 이름만 &str로 추출한다.
        let dir_name = dir_entry.file_name();
        let dir_name = dir_name.to_str().unwrap();

        // 5. 해당 시즌 별 디렉터리 이름은 list.season(Vec) 필드에 저장한다.
        list.seasons.push(dir_name.to_owned());

        // 6. 시즌 별 디렉터리 내에 있는 video 파일을 찾기 위해 Path를 /src/resources/{season}으로 수정한다.
        let season_path = format!("{}/{}", PATH, dir_name);
        // 7. 시즌 별 디렉터리를 읽어들인다.
        let mut entries = fs::read_dir(season_path).await?;

        // 8. (3)과 같은 방법으로 디렉터리를 읽어들인다.
        while let Ok(Some(entry)) = entries.next_entry().await {
            let f_name = entry.file_name();
            let f_name = f_name.to_str().unwrap();
            let parse_episode: Vec<&str> = f_name.split('-').collect();
            let episode = parse_episode[1].replace(".mp4", "");

            // 9. 해당 시즌 내에 존재하는 에피소드를 저장한다.
            animeList.insert(&dir_name, &episode);
        }
    }

    // 10. Result 형식으로 안전하게 Json(list)를 반환한다.
    Ok(Json(animeList))
}

pub async fn get_list_handler() -> Result<impl IntoResponse, Infallible> {
    let get_video_list = video_list_handler().await;

    match get_video_list {
        Ok(res) => {
            let json_str = serde_json::to_string(&res.0).unwrap();
            let resp: Response<Body> = Response::new(json_str.into());
            Ok(resp.into_response())
        }
        Err(e) => {
            dbg!(&e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
