#![allow(unused)]
extern crate ffmpeg_next as ffmpeg;

use axum::extract::Path;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::codec::context::Context as Codec_Context;
use ffmpeg::util::frame::video::Video;
use image::{ImageBuffer, Rgb};
use tokio::{io, fs};
use std::fs::File;
use std::io::Write;
use crate::error::error::ServerError;
use crate::list::list::PATH as DIR_PATH;

async fn frame_handler(season: &str, episode: &str) -> Result<(), ServerError> {
    ffmpeg::init().unwrap();
    let file_path = format!("{}/{}-{}.mp4", DIR_PATH, season, episode);

    if let Ok(mut ictx) = input(&file_path) {
        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;

        let video_stream_index = input.index();

        let context_decoder = Codec_Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        let mut frame_index = 100;

        let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
            let mut decoded = Video::empty();
            if decoder.receive_frame(&mut decoded).is_ok() {
                if frame_index == decoded.coded_number() { // 특정 프레임 번호를 지정합니다.
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    save_file(&rgb_frame, frame_index).unwrap();
                }
            }
            Ok(())
        };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
    }

    Ok(())
}

fn calculate_linesize(width: usize, pix_fmt: Pixel) -> usize {
    match pix_fmt {
        Pixel::RGB24 => width * 3,
        _ => unimplemented!(),
    }
}

fn save_file(frame: &Video, index: usize) -> Result<(), ServerError> {
    let mut buffer = ImageBuffer::<Rgb<u8>, _>::new(
        frame.width(),
        frame.height()
    );
    let data = frame.data(0);
    let linesize = calculate_linesize(frame.width() as usize, frame.format()) as u32;
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let r = data[(y * linesize + (x * 3) as u32) as usize];
            let g = data[(y * linesize + (x * 3) as u32 + 1) as usize];
            let b = data[(y * linesize + (x * 3) as u32 + 2) as usize];
            *pixel = Rgb([r, g, b]);
    }

    let output_file_path = format!("src/images/frame{}.png", index);
    
    match File::open(&output_file_path) {
        Err(_) => {
            match buffer.save(&output_file_path) {
                Err(e) => eprintln!("Error writing file: {}", e),
                Ok(()) => println!("Done."),
            }
        },
        Ok(_) => ()
    }

    Ok(())
}

pub async fn view_image_handler(Path((info)): Path<String>) {
    let data: Vec<&str> = info.split('-').collect();
    let season = data[0];
    let episode = data[1];
    dbg!((season, episode));
    let res = dbg!(frame_handler(&season, &episode).await);
}