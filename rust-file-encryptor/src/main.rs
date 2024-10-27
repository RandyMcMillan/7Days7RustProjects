mod cli;
mod encryption;
mod io;

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();
    let key = encryption::get_key();
    let mut data = io::read_file(&args.input)?;

    if args.decrypt {
        encryption::encrypt_decrypt(&mut data, key);
        let output_path = args.input.with_extension("decrypted");
        io::write_file(&output_path, &data)?;
        println!("Decrypted file saved to: {:?}", output_path);
    } else {
        encryption::encrypt_decrypt(&mut data, key);
        let output_path = args.input.with_extension("encrypted");
        io::write_file(&output_path, &data)?;
        println!("Encrypted file saved to: {:?}", output_path);
    }

    Ok(())
}
