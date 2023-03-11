use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use std::convert::Infallible;
use tokio::{fs, io};

use super::list::Video;

// video file의 path
pub const PATH: &str = "src/resources";

pub async fn video_list_handler() -> Result<Json<Vec<Video>>, io::Error> {
    // 1. Path의 Directory를 읽어들인다.
    let mut entries = fs::read_dir(PATH).await?;
    // 2. 최종적으로 반환할 VideoList를 생성한다.
    let mut video_list: Vec<Video> = Vec::new();

    while let Ok(Some(anime_entry)) = entries.next_entry().await {
        //애니 목록
        // 3. 디렉터리 내에 있는 애니 디렉터리를 하나씩(anime_entry) 읽어들인다.
        let anime = anime_entry.file_name(); //애니이름
        let anime = anime.to_str().unwrap();

        let anime_path = format!("{}/{}", PATH, anime);

        let mut entries = fs::read_dir(anime_path.to_owned()).await?;

        let mut video = Video::new();
        video.name = anime.to_string();

        while let Ok(Some(season_entry)) = entries.next_entry().await {
            //시즌 목록
            // 4. 변수 셰도잉으로 directory의 이름만 &str로 추출한다.
            let season = season_entry.file_name();
            let season = season.to_str().unwrap();

            // 5. 시즌 별 디렉터리 내에 있는 video 파일을 찾기 위해 Path를 /src/resources/{anime}/{season}으로 수정한다.
            let season_path = format!("{}/{}", anime_path.to_owned(), season);
            // 6. 시즌 별 디렉터리를 읽어들인다.
            let mut entries = fs::read_dir(season_path).await?;
            // 7. (3)과 같은 방법으로 디렉터리를 읽어들인다.
            while let Ok(Some(episode_entry)) = entries.next_entry().await {
                //에피소드 목록
                let f_name = episode_entry.file_name();
                let f_name = f_name.to_str().unwrap();
                let parse_episode: Vec<&str> = f_name.split('-').collect();
                let episode = parse_episode[1].replace(".mp4", "");

                // 9. 해당 시즌 내에 존재하는 에피소드를 저장한다.
                video.push_episode(season, &episode)
            }
        }
        video_list.push(video);
    }

    // 10. Result 형식으로 안전하게 Json(list)를 반환한다.
    Ok(Json(video_list))
}

pub async fn get_list_handler() -> Result<impl IntoResponse, Infallible> {
    let get_video_list = video_list_handler().await;

    match get_video_list {
        Ok(res) => Ok(res.into_response()),
        Err(e) => {
            dbg!(&e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
