use ffmpeg_next as ffmpeg;
use ffmpeg::format::{input, Pixel};
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use image::RgbImage;

fn rgb_frame_to_image(frame: &Video, w: u32, h: u32) -> RgbImage {
    let data = frame.data(0);
    let stride = frame.stride(0);
    let mut img = RgbImage::new(w, h);
    for y in 0..h as usize {
        let row = &data[y * stride..y * stride + w as usize * 3];
        for x in 0..w as usize {
            let i = x * 3;
            img.put_pixel(x as u32, y as u32, image::Rgb([row[i], row [i + 1], row[i + 2]]));
        }
    }
    img
}

pub fn decode<F: FnMut(RgbImage)>(path: &str, mut on_frame: F) -> anyhow::Result<()> {
    ffmpeg::init()?;
    let mut ictx = input(&path).expect("Failed to open input file");
    let stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or_else(|| anyhow::anyhow!("no video stream"))?;

    let stream_idx = stream.index();

    let ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters()).expect("Failed to create codec context");
    let mut decoder = ctx.decoder().video().expect("Failed to create video decoder");
    let (w, h) = (decoder.width(), decoder.height());

    let mut scaler = Context::get(
        decoder.format(), w, h,
        Pixel::RGB24, w, h,
        Flags::BILINEAR,
    ).expect("Failed to create scaler");

    for (s, packet) in ictx.packets() {
        if s.index() != stream_idx {
            continue;
        }
        decoder.send_packet(&packet)?;
        let mut frame = Video::empty();
        while decoder.receive_frame(&mut frame).is_ok() {
            let mut rgb = Video::new(Pixel::RGB24, w, h);
            scaler.run(&frame, &mut rgb).expect("Failed to scale frame");
            on_frame(rgb_frame_to_image(&rgb, w, h));
        }
    }
    decoder.send_eof()?;
    let mut frame = Video::empty();
    while decoder.receive_frame(&mut frame).is_ok() {
        let mut rgb = Video::empty();
        scaler.run(&frame, &mut rgb).expect("Failed to scale frame");
        on_frame(rgb_frame_to_image(&rgb, w, h));
    }
    Ok(())
}