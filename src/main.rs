use c2p::convert_img_to_c2p;
use std::{env::args, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        convert_img_to_c2p(&args[1], (&args[2]).to_string())?;
    }
    Ok(())
}
