use std::io::{stdin, stdout, Write};
use termion::color;
use termion::{cursor::DetectCursorPos, event::Key, input::TermRead, raw::IntoRawMode};

use tedit::rope::*;

fn main() {
    let terminal = stdout().into_raw_mode();
    let mut stdout = terminal.unwrap();

    let mut current_line = 1;
    let mut max_line = 1;

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    let stdin = stdin().keys();
    line_numbering(&mut stdout, current_line, 1);

    stdout.flush().unwrap();

    for c in stdin {
        if let Ok(input) = c {
            match input {
                Key::Char('\n') => {
                    max_line += 1;
                    write!(stdout, "\n\r").unwrap();
                    line_numbering(&mut stdout, current_line, max_line);
                    current_line += 1;
                    write!(stdout, "{}", termion::cursor::Goto(8, current_line as u16)).unwrap();
                }
                Key::Down => {}
                Key::Up => {
                    current_line -= 1;
                    write!(stdout, "{}", termion::cursor::Up(1)).unwrap();
                }
                Key::Right => {}
                Key::Left => {}
                Key::Char('q') => break,
                Key::Char(';') => {
                    let (x, y) = stdout.cursor_pos().unwrap();
                    write!(stdout, "{}", termion::cursor::Down(20)).unwrap();
                    write!(stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
                }
                Key::Char(c) => {
                    write!(stdout, "{}", c).unwrap();
                }
                Key::Backspace => {}
                _ => {}
            }
        }
        stdout.flush().unwrap();
    }
}

pub fn line_numbering(
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    current_line: usize,
    max_line: usize,
) {
    for i in current_line..=max_line {
        let s = format!("{: >6}", i);
        write!(stdout, "{}", termion::cursor::Goto(1, i as u16)).unwrap();
        write!(
            stdout,
            "{}{}{}{}",
            color::Fg(color::LightWhite),
            s,
            color::Fg(color::Reset),
            ' '
        )
        .unwrap();
    }
}
