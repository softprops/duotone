use raster::{filter, editor};
use raster::{Color, Image, BlendMode, PositionMode};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut img = raster::open("saysueme.jpg").unwrap();
    filter::grayscale(&mut img).unwrap();
    let mut light = Image::blank(img.width, img.height);
    editor::fill(&mut light, Color::hex("#ffff00").unwrap()).unwrap();
    let mut dark = Image::blank(img.width, img.height);
    editor::fill(&mut dark, Color::hex("#0000ff").unwrap()).unwrap();
    let blended = editor::blend(&img, &light, BlendMode::Multiply, 100.0, PositionMode::Center, 0, 0).unwrap();
    let result = editor::blend(&blended, &dark, BlendMode::Screen, 80.0, PositionMode::Center, 0, 0).unwrap();

    raster::save(&result, "target/saysueme.jpg").unwrap();
    Ok(())
}
