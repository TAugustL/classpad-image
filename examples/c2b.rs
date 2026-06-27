use classpad_image::{convert_c2b_to_imgs, convert_imgs_to_c2b};
use std::{error::Error, path::Path};

// Converts some images to C2B, or converts a C2B into its images.

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments supplied!");
        eprintln!("Usage: .c2b (IMAGES PATH) (DESTINATION PATH)");
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
        "png" | "jpg" | "bmp" | "webp" => {
            convert_imgs_to_c2b(args[1..args.len() - 1].to_vec(), args.last().unwrap())?
        }
        "c2b" => convert_c2b_to_imgs(image_path, destination)?,
        "c2p" => eprintln!("This example uses only C2B! For C2P conversion, see examples/c2p.rs."),

        _ => {
            eprintln!("File extension not supported!");
            eprintln!("Supported types: PNG, JPG, BMP, WEBP, C2B");
        }
    }

    Ok(())
}
