use classpad_image::{convert_c2p_to_img, convert_img_to_c2p};
use std::{error::Error, path::Path};

// Converts a image to C2P, or converts a C2P into an image.

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments supplied!");
        eprintln!("Usage: .c2p (IMAGE PATH) (DESTINATION PATH)");
        return Ok(());
    }

    let image_path: &Path = args[1].as_ref();
    let extension: &str = image_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let destination: &str = &args[2];

    match extension.to_lowercase().as_str() {
        "png" | "jpg" | "bmp" | "webp" => convert_img_to_c2p(image_path, destination)?,
        "c2p" => convert_c2p_to_img(image_path, destination)?,
        "c2b" => eprintln!("This example uses only C2P! For C2B conversion, see examples/c2b.rs."),

        _ => {
            eprintln!("File extension not supported!");
            eprintln!("Supported types: PNG, JPG, BMP, WEBP, C2P");
        }
    }

    Ok(())
}
