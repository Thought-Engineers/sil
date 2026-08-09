pub mod fractalzip;
pub mod pack;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        eprintln!("Usage: sil <compress|decompress> <input_file> <output_file>");
        std::process::exit(1);
    }

    let command = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let fz = fractalzip::FractalZip::new();

    match command.as_str() {
        "compress" => {
            if let Err(e) = compress_file(&fz, input_file, output_file) {
                eprintln!("Error compressing: {}", e);
                std::process::exit(1);
            }
            println!("Successfully compressed.");
        }
        "decompress" => {
            if let Err(e) = decompress_file(&fz, input_file, output_file) {
                eprintln!("Error decompressing: {}", e);
                std::process::exit(1);
            }
            println!("Successfully decompressed.");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}

fn compress_file(fz: &fractalzip::FractalZip, input: &str, output: &str) -> std::io::Result<()> {
    let mut data = String::new();
    let mut file = File::open(input)?;
    file.read_to_string(&mut data)?;

    let stream = fz.compress(&data);
    let packed = pack::pack(&stream);

    let mut out = File::create(output)?;
    
    // Write length of stream as u64 in little-endian
    let length = stream.len() as u64;
    out.write_all(&length.to_le_bytes())?;
    
    // Write packed data
    out.write_all(&packed)?;

    Ok(())
}

fn decompress_file(fz: &fractalzip::FractalZip, input: &str, output: &str) -> std::io::Result<()> {
    let mut in_file = File::open(input)?;
    
    // Read length
    let mut length_buf = [0u8; 8];
    in_file.read_exact(&mut length_buf)?;
    let stream_len = u64::from_le_bytes(length_buf) as usize;

    // Read packed data
    let mut packed = Vec::new();
    in_file.read_to_end(&mut packed)?;

    let stream = pack::unpack(&packed, stream_len);
    let recovered = fz.decompress(&stream);

    let mut out_file = File::create(output)?;
    out_file.write_all(recovered.as_bytes())?;

    Ok(())
}
