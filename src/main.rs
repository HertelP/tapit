use std::io::{stdin, stdout, Write, Read};
use termion::raw::IntoRawMode;

fn main() {
    let terminal = stdout().into_raw_mode();
    let stdout = terminal.unwrap();

    let mut stdin = termion::async_stdin().bytes();

    loop {
        let b = stdin.next();
        if let Some(Ok(input)) = b {
            match input as char {
                'q' => break,
                _ => println!("{}", input as char),
            }
        }
        stdout.lock().flush().unwrap();
    }
}
