use image::{image_dimensions, open, Pixel};
use rand::Rng;
use std::time::Instant;
use std::{env, fs};

fn main() -> std::io::Result<()> {
    let startup = Instant::now();
    let args: Vec<String> = env::args().skip(1).collect();

    let layers = match args.len() {
        2.. => args[1].parse().unwrap(),
        _ => 1,
    };

    let image = open(args[0].clone()).unwrap().into_rgba16();

    let _ = fs::remove_dir_all("./output");

    if args.len() < 2 {
        println!("complete in {:#?}", startup.elapsed());
        return Ok(());
    }

    let _ = fs::create_dir("./output");

    let mut layersvec: Vec<(f32, f32, f32, f32)> = Vec::new();

    for _ in 0..layers {
        let mut rng0 = rand::thread_rng();
        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();
        layersvec.push((rng0.gen(), rng1.gen(), rng2.gen(), 1.0)); 
    }
    println!("generated color set");

    let (w, h) = image_dimensions(args[0].clone()).unwrap();

    println!("{w}x{h} pixel buffer: {} in {:#?}", image.len(), startup.elapsed());

    for i in 0..layers {
        let frametime = Instant::now();
        let mut pixels: Vec<u16> = Vec::new();
        for x in image.pixels() {
            let pixel = x.channels4();
            pixels.push((pixel.0 as f32 * layersvec[i].0) as u16);
            pixels.push((pixel.1 as f32 * layersvec[i].1) as u16);
            pixels.push((pixel.2 as f32 * layersvec[i].2) as u16);
            pixels.push((pixel.3 as f32 * layersvec[i].3) as u16);
        }
        let i = i + 1;
        let path = format!("./output/{}.png", i);
        let layer = image::DynamicImage::ImageRgba16(
            image::ImageBuffer::from_raw(w, h, pixels).unwrap(),
        );
        layer.save(path).unwrap();
        println!("completed layer: {} in {:#?}", i, frametime.elapsed());
    }

    println!("complete in {:#?}", startup.elapsed());

    Ok(())
}

