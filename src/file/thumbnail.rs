extern crate ffmpeg_next as ffmpeg;
extern crate image;

use std::path::PathBuf;

use ffmpeg::format;
use ffmpeg::media::Type;
use ffmpeg::codec::Context as CodecContext;
use ffmpeg::software::scaling::{context::Context as ScalerContext, flag::Flags};
use ffmpeg::util::frame::video::Video as VideoFrame;
use image::{ImageBuffer, RgbImage};

use crate::utils::logger::*;

use super::VideoInfo;

pub fn generate_thumbnail(video: &VideoInfo) -> VideoFrame {
    ffmpeg::init().unwrap();

    let mut context = format::input(&video.path).unwrap();

    let input = context.streams().best(Type::Video)
        .ok_or("Could not find a video stream").unwrap();
    let video_stream_index = input.index();

    // Get the codec context for decoding the video stream
    let codec_params = input.parameters();
    let mut decoder = CodecContext::from_parameters(codec_params).unwrap().decoder().video().unwrap();

    let mut scaler = ScalerContext::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    ).unwrap();

    let mut decoded_frame = VideoFrame::empty();
    let mut rgb_frame = VideoFrame::empty();

    // Iterate through the packets and decode the first video frame
    for (stream, packet) in context.packets() {
        if stream.index() == video_stream_index {
            // Send the packet to the decoder
            let _ = decoder.send_packet(&packet).unwrap();

            // Receive the decoded frame
            while decoder.receive_frame(&mut decoded_frame).is_ok() {
                // Convert the frame to RGB
                scaler.run(&decoded_frame, &mut rgb_frame).unwrap();

                // The first frame is now in `rgb_frame`
                info(&format!("First frame decoded: {}x{}, format: {:?}",
                    rgb_frame.width(),
                    rgb_frame.height(),
                    rgb_frame.format()));

                // Stop after processing the first frame
                break;
            }

            // Stop after reading the first frame
            break;
        }
    }

    return rgb_frame;
}

pub fn save_thumbnail_to_file(frame: VideoFrame, file_path: PathBuf) {
    // Get the frame's width, height, and stride (line size)
    let width = frame.width() as u32;
    let height = frame.height() as u32;
    let stride = frame.stride(0);

    // Extract the data as RGB bytes
    let data = frame.data(0);

    // Create an empty buffer for the image (width * height * 3 because it's RGB)
    let mut buffer: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);

    // Copy the frame's data into the buffer row by row
    for y in 0..height {
        let offset = (y as usize) * stride;
        buffer.extend_from_slice(&data[offset..offset + (width * 3) as usize]);
    }

    // Create an image buffer from the raw RGB data
    let img: RgbImage = ImageBuffer::from_raw(width, height, buffer).ok_or("Failed to create image buffer").unwrap();

    // Save the image to the specified file
    img.save(&file_path).unwrap();

    info(&format!("Saved frame as {}", file_path.to_string_lossy()));
}