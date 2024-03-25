extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::env;
use std::fs;
use std::io::{self, Cursor, ErrorKind, Read, Write};
use std::vec::Vec;

const BUP_BEGIN_SKIP: u64 = 284;
const MTRK_TOKEN_VALUE: i32 = 1297379947; // ASCII string MTrk
const MTRK_HEADER_SIZE: u64 = 8; // ASCII string MTrk
const BUP_FIRST_SEGMENT_LENGTH: u64 = 504;
const BUP_TRASH_SEGMENT_LENGTH: u64 = 512;

const MIDI_HEADER_TOKEN: u32 = 0x4D546864;
const MIDI_HEADER_SIZE: u32 = 6;
const MIDI_HEADER_FORMAT: u16 = 0;
const MIDI_HEADER_NUM_TRACKS: u16 = 1;
const MIDI_HEADER_DIVISION: u16 = 0x0060;

fn print_help() {
    println!("Usage: ypd131bup2midi FILE");
}

fn extract_midi_data(file_name: &std::string::String) -> std::io::Result<Vec<u8>> {
    // check if file exists
    let metadata = match fs::metadata(&file_name) {
        Err(why) => {
            println!("File {} not found", file_name);
            return Err(why);
        }
        Ok(m) => m,
    };
    if !metadata.is_file() {
        println!("Input is not a file");
        return Err(io::Error::new(ErrorKind::Other, "Input is not a file"));
    }

    let file_contents = match fs::read(&file_name) {
        Err(why) => return Err(why),
        Ok(file) => file,
    };

    let mut freader = Cursor::new(file_contents);

    freader.set_position(BUP_BEGIN_SKIP);

    let chunk_token = freader.read_i32::<BigEndian>()?;
    let chunk_data_length = freader.read_u32::<BigEndian>()? as u64;
    if chunk_token != MTRK_TOKEN_VALUE {
        return Err(io::Error::new(
            ErrorKind::Other,
            "Unexpected BUP file layout",
        ));
    }

    let mut midi_data: Vec<u8> = vec![0; chunk_data_length as usize];

    if chunk_data_length as u64 <= BUP_FIRST_SEGMENT_LENGTH {
        // Midi data is not interrupted by a 512 byte segment of other data.
        //freader.read(midi_data);
        freader.read_exact(&mut midi_data[0..(chunk_data_length - 1) as usize])?;
    } else {
        // Midi data will be interrupted.
        freader.read_exact(&mut midi_data[0..(BUP_FIRST_SEGMENT_LENGTH - 1) as usize])?;
        // jump over trash segment
        freader.set_position(
            BUP_BEGIN_SKIP + MTRK_HEADER_SIZE + BUP_FIRST_SEGMENT_LENGTH + BUP_TRASH_SEGMENT_LENGTH,
        );
        // read remaining data
        freader.read_exact(
            &mut midi_data[BUP_FIRST_SEGMENT_LENGTH as usize..(chunk_data_length - 1) as usize],
        )?;
    }
    Ok(midi_data)
}

fn generate_output_filename(input_name: &std::string::String) -> std::string::String {
    // we don't really care if the input name ends with .bup
    let dot_offset = input_name.rfind('.').unwrap_or(input_name.len());
    let mut output_name = input_name.clone();
    output_name.truncate(dot_offset);
    output_name + ".mid"
}

fn write_midi_file(
    output_filename: &std::string::String,
    midi_data: &Vec<u8>,
) -> std::io::Result<()> {
    let mut file = fs::File::create(output_filename)?;
    file.write_u32::<BigEndian>(MIDI_HEADER_TOKEN).unwrap();
    file.write_u32::<BigEndian>(MIDI_HEADER_SIZE).unwrap();
    file.write_u16::<BigEndian>(MIDI_HEADER_FORMAT).unwrap();
    file.write_u16::<BigEndian>(MIDI_HEADER_NUM_TRACKS).unwrap();
    file.write_u16::<BigEndian>(MIDI_HEADER_DIVISION).unwrap();
    file.write_i32::<BigEndian>(MTRK_TOKEN_VALUE).unwrap();
    file.write_u32::<BigEndian>(midi_data.len() as u32).unwrap();
    file.write(midi_data).unwrap();
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("Hi. This is ypd131bup2midi.");

    if args.len() == 1 {
        print_help();
        return Ok(());
    } else if args.len() > 2 {
        println!("Too many arguments.");
        return Ok(());
    }

    let midi_data: Vec<u8> = match extract_midi_data(&args[1]) {
        Ok(data) => data,
        Err(_e) => {
            println!("Failed to extract midi data from file!");
            return Ok(());
        }
    };

    let output_filename = generate_output_filename(&args[1]);

    match write_midi_file(&output_filename, &midi_data) {
        Ok(()) => println!("Wrote output to {}", output_filename),
        Err(_e) => println!("Failed to write midi data to {}", output_filename),
    }
    Ok(())
}
