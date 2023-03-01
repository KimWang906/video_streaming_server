#![allow(unused)]
extern crate image;
extern crate ffmpeg_next as ffmpeg;

use std::fs::File;
use axum::extract::Path;
use image::{ImageBuffer, Rgb};
use ffmpeg::{
    format::{input, Pixel},
    media::Type::Video as VideoType, 
    frame::Video, software::scaling::{Context, Flags}, dict
};
use tokio::{io, fs};
use crate::list::list::PATH as DIR_PATH;

async fn frame_handler(season: &str, episode: &str) -> io::Result<()> {
    let file_path = format!("{}/{}-{}.mp4", DIR_PATH, season, episode);
    let mut ictx = input(&file_path).unwrap();
    let input = ictx
        .streams()
        .best(VideoType)
        .unwrap();

    let mut decoder = input.codec().decoder().video()?;
    decoder.set_parameters(input.parameters())?;
    
    let mut frame = Video::empty();
    let mut scaler = match decoder.format() {
        Pixel::RGB24 => Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR
        ),
        _ => unimplemented!(),
    }?;

    let mut num_frames = 0;

    // while ictx. {
        
    // }

    Ok(())
}

pub async fn view_image_handler(Path(season): Path<String>, Path(episode): Path<String>) {
    let res = frame_handler(&season, &episode).await;
}