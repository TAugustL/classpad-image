use classpad_image::{
    convert_c2b_to_imgs, convert_c2p_to_img, convert_img_to_c2b, convert_img_to_c2p,
};
use std::{env::args, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments supplied! Enter 'help' to see possible parameters.");
        return Ok(());
    }
    match args[1].as_str() {
        "c2p" => convert_img_to_c2p(&args[2], args[3].to_string())?, // cargo run c2p path/to/image.png destination/path/test.c2b
        "c2b" => {
            // cargo run c2p path/to/image2.png destination/path/test.c2b
            let mut path_vec: Vec<String> = Vec::new();
            let mut args_parts = args;
            let dest = args_parts.pop().expect("Failed to get destination!");
            args_parts.remove(0);
            args_parts.remove(0);
            for arg in args_parts {
                path_vec.push(arg);
            }
            convert_img_to_c2b(path_vec, dest)?
        }
        "imgp" => convert_c2p_to_img(&args[2], args[3].to_string())?, // cargo run imgp path/to/image.c2p destination/path/test.png
        "imgb" => convert_c2b_to_imgs(&args[2], args[3].to_string())?, // cargo run imgb path/to/image.c2b destination/path/test.png
        _ => eprintln!("Unknown argument! Enter 'help' to see possible parameters."),
    }
    Ok(())
}
