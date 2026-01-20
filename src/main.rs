mod parser_old;
mod parser_tizen_1_8;
mod common;

use clap::Parser;
use std::fs::{File};
use std::io::{Read};

use crate::parser_old::{parse_ouith_blob};
use crate::parser_tizen_1_8::{parse_blob};

#[derive(Parser, Debug)]
struct Args {
    mode: String,
    input_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let in_file = args.input_file;
    let mode = args.mode;
    println!("Input file: {}", in_file);

    let mut file = File::open(in_file)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if mode == "old" {
        parse_old(&data)?;
    }
    else if mode == "tizen_1.8" {
        parse_tizen_1_8(&data)?;
    }
    else {
        println!("Unknown mode {}", mode);
    }


    Ok(())
}

fn parse_tizen_1_8(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let (items, info) = parse_blob(&data)?;

    println!("\nParsed MSD items:");
    for item in items {
        println!("ID {} - {}",
                item.item_id, item.name);
        
        //do if crc32 checksummed
        if let Some(crc32_hash) = item.crc32_hash {
            println!("- CRC32: {:02x}", crc32_hash);
        } else {
            println!("- CRC32: False");
        }

        //do if aes encrypted
        if item.aes_encryption {
            println!("- Encrypted: True");
            println!("- Salt: {}", hex::encode(&item.aes_salt.unwrap()));
        } else {
            println!("- Encrypted: False")
        }

        println!();
    }

    println!("Parsed MSD info:");
    if let Some(info) = info {
        println!("{} {}.{} {}/{}/{}", info.name(), info.major_ver, info.minor_ver, info.date_day, info.date_month, info.date_year);
    } else {
        println!("\nDid not get MSD info.");
    }

    Ok(())
}

fn parse_old(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let (items, info) = parse_ouith_blob(&data)?;

    println!("\nParsed MSD items:");
    for item in items {
        println!("ID {} - {}, Type: {}, All size: {}, Heading size: {}, Data size: {}",
                item.item_id, item.name, item.item_type, item.all_size, item.heading_size, item.data_size);

        //types
        if item.item_type == 0x0A {
            println!("- Type: Partition");
        }
        if item.item_type == 0x0B {
            println!("- Type: File");
        }
        if item.item_type == 0x11 {
            println!("- Type: CMAC Data");
        }
        
        //do if aes encrypted
        if item.aes_encryption {
            println!("- Encrypted: True");
        } else {
            println!("- Encrypted: False")
        }

        //do if crc32 checksummed
        if let Some(crc32_hash) = item.crc32_hash {
            println!("- CRC32: {:02x}", crc32_hash);
        } else {
            println!("- CRC32: False");
        }

        //do if secure hashed
        if let Some(secure_hash) = item.secure_hash {
            println!("- Secure hash: {}", hex::encode(&secure_hash));
        } else {
            println!("- Secure hash: False");
        }

        println!();
    }

    println!("Parsed MSD info:");
    if let Some(info) = info {
        println!("{} {}.{} {}/{}/20{}", info.name(), info.major_ver, info.minor_ver, info.date_day, info.date_month, info.date_year);
    } else {
        println!("\nDid not get MSD info.");
    }

    Ok(())
}
