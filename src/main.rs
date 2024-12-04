use std::env::args;
use image::ImageReader;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    if args().len() != 3 {
        eprint!("Usage: `source` `target`");
        return Ok(());
    }
    
    let img = ImageReader::open(args().nth(1).unwrap())?.with_guessed_format()?.decode()?;
    img.save_with_format(args().nth(2).unwrap(), image::ImageFormat::Ico)?;

    Ok(())
}