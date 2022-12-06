use anyhow::Result;
use std::io::{stdin, BufRead, BufReader};

fn read(mut reader: impl BufRead) -> Result<String> {
    let mut packet = String::new();
    reader.read_to_string(&mut packet)?;

    Ok(packet.trim().to_owned())
}

fn find_all_unique_piece(packet: &str, piece_len: usize) -> Option<usize> {
    for (idx, maybe_unique) in packet.as_bytes().windows(piece_len).enumerate() {
        let is_unique = !(0..piece_len)
            .into_iter()
            .any(|idx| maybe_unique[..idx].contains(&maybe_unique[idx]));

        if is_unique {
            return Some(idx + piece_len);
        }
    }

    None
}

fn packet_prelude_position(packet: &str) -> Option<usize> {
    find_all_unique_piece(packet, 4)
}

fn message_start_position(packet: &str) -> Option<usize> {
    find_all_unique_piece(packet, 14)
}

fn print_result(result: Option<usize>, result_type: &'static str) {
    match result {
        Some(position) => {
            println!(
                "{} characters needs to be processed before {} is detected.",
                position, result_type
            );
        }
        None => println!("Couldn't find {} in the packet.", result_type),
    }
}

fn main() -> Result<()> {
    let packet = read(BufReader::new(stdin()))?;

    print_result(packet_prelude_position(&packet), "prelude");
    print_result(message_start_position(&packet), "message start");

    Ok(())
}
