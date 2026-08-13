pub mod fractalzip;
pub mod pack;

use std::env;
use std::fs::File;
use std::io::{Read, Write};

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
    let mut in_file = File::open(input)?;
    let mut out_file = File::create(output)?;
    
    let mut buf = vec![0u8; 1024 * 1024]; // 1MB chunks
    
    loop {
        let n = in_file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        
        let data = String::from_utf8_lossy(&buf[..n]).to_string();
        let stream = fz.compress(&data);
        let packed = pack::pack(&stream);
        
        let length = stream.len() as u64;
        out_file.write_all(&length.to_le_bytes())?;
        out_file.write_all(&packed)?;
    }

    Ok(())
}

fn decompress_file(fz: &fractalzip::FractalZip, input: &str, output: &str) -> std::io::Result<()> {
    let mut in_file = File::open(input)?;
    let mut out_file = File::create(output)?;
    
    loop {
        let mut length_buf = [0u8; 8];
        match in_file.read_exact(&mut length_buf) {
            Ok(_) => {},
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
        
        let stream_len = u64::from_le_bytes(length_buf) as usize;
        let packed_len = (stream_len + 3) / 4;
        
        let mut packed = vec![0u8; packed_len];
        in_file.read_exact(&mut packed)?;

        let stream = pack::unpack(&packed, stream_len);
        let recovered = fz.decompress(&stream);

        out_file.write_all(recovered.as_bytes())?;
    }

    Ok(())
}
