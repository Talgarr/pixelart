use clap::Parser;
use image::{DynamicImage, ImageReader, RgbImage};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'i')]
    input: String,

    #[arg(short = 's')]
    pixel_size: u32,
}

fn image_to_low_res(rgb: &RgbImage, pixel_size: u32) -> RgbImage {
    let width = rgb.width() / pixel_size;
    let heigh = rgb.height() / pixel_size;

    let mut pixelart = RgbImage::new(width, heigh);
    for (i, pixel) in pixelart.pixels_mut().enumerate() {
        let x = ((i as u32) * pixel_size) % rgb.width();
        let y = ((i as u32) * pixel_size) / rgb.width() * pixel_size;
        let color = rgb.get_pixel(x, y);
        pixel[0] = color[0];
        pixel[1] = color[1];
        pixel[2] = color[2];
    }
    pixelart
}

fn read_image(path: &str) -> DynamicImage {
    let image = ImageReader::open(path)
        .expect(&format!("Failed to open file: {}", path))
        .decode()
        .expect("Failed to decode image");
    image
}

fn pixels_to_ansi(img: &RgbImage) -> String {
    let mut s = String::new();

    for (i, c) in img.pixels().enumerate() {
        let r = c[0] as u32 / 43;
        let g = c[1] as u32 / 43;
        let b = c[2] as u32 / 43;
        let color = 16 + 36 * r + 6 * g + b;
        if i as u32 % img.width() == 0 {
            s.push('\n');
        }
        s.push_str(format!("\x1b[38;5;{color}m██").as_str());
    }
    s
}

struct PixelSize {
    pixel_size: u32,
    width: u32,
    scale: u32,
}

impl PixelSize {
    fn new(pixel_size: u32, width: u32) -> Self {
        PixelSize {
            pixel_size,
            width,
            scale: 1,
        }
    }
}

impl Iterator for PixelSize {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.pixel_size * self.scale;
        self.scale *= if cur % 2 == 0 { 2 } else { 3 };
        if cur > self.width { None } else { Some(cur) }
    }
}

fn main() {
    let args = Cli::parse();
    let im = read_image(&args.input);
    let rgb = im.to_rgb8();
    for size in PixelSize::new(args.pixel_size, rgb.width()) {
        let px = image_to_low_res(&rgb, size);
        let ansi = pixels_to_ansi(&px);
        println!("{}", ansi);
    }
    // let _ = px.save("result.png");
}
