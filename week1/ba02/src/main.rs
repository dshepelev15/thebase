use std::io::{self, Read};

fn main() {
    let mut lines_count = 0;
    let mut words_count = 0;
    let mut bytes_count = 0;


    let mut word_started = false;
    for byte in io::stdin().bytes() {
        bytes_count += 1;
        let byte = byte.unwrap();

        if byte.is_ascii_whitespace() {
            word_started = false;

            if byte == b'\n' {
                lines_count += 1;
            }
            continue;
        }
        
        if !word_started {
            words_count += 1;
            word_started = true;
        }
    }

    println!("{} {} {}", lines_count, words_count, bytes_count);

}
