use std::io;
use std::io::{BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        // URL encode
        let line = line.unwrap();
        let line = urlencoding::encode(&line);
        writeln!(stdout, "{}", line).unwrap();
    }
}
