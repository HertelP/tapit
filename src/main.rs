use std::io::{stdin, stdout, Write};
use termion::color;
use termion::cursor::DetectCursorPos;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub fn init() -> termion::raw::RawTerminal<std::io::Stdout> {
    let terminal = stdout().into_raw_mode();
    let mut stdout = terminal.unwrap();
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    write!(
        stdout,
        "   {}1{} ",
        color::Fg(color::LightWhite),
        color::Fg(color::Reset)
    )
    .unwrap();

    stdout.flush().unwrap();

    stdout
}

pub struct Line {
    number: usize,
    text: String,
}

fn main() {
    let mut stdout = init();

    let mut lines: Vec<Line> = Vec::new();

    let stdin = stdin().keys();
    let mut current_line = 0;
    let mut buffer = String::new();

    for c in stdin {
        if let Ok(input) = c {
            match input {
                Key::Char('\n') => {
                    if lines.is_empty() || lines.len() == current_line {
                        lines.push(Line {
                            number: current_line,
                            text: buffer.trim_end().to_string(),
                        });
                    } else {
                        lines[current_line] = Line {
                            number: current_line,
                            text: buffer.trim_end().to_string(),
                        };
                    }
                    buffer.clear();
                    current_line += 1;
                    write!(stdout, "\r\n").unwrap();
                    write!(
                        stdout,
                        "   {}{}{} ",
                        color::Fg(color::LightWhite),
                        current_line + 1,
                        color::Fg(color::Reset)
                    )
                    .unwrap();
                }
                Key::Down => {
                    if current_line >= lines.len()-1 {
                        continue;
                    }
                    current_line += 1;
                    let size = lines[current_line].text.chars().count();
                    write!(stdout, "{}", termion::cursor::Goto((size+6) as u16,(current_line+1) as u16)).unwrap();
                    buffer = lines[current_line].text.clone();
                }
                Key::Up => {
                    if current_line == 0 {
                        continue;
                    }
                    if current_line >= lines.len() {
                        lines.push(Line {
                            number: current_line,
                            text: buffer.trim_end().to_string(),
                        });
                    }
                    current_line -= 1;
                    let size = lines[current_line].text.chars().count();
                    write!(stdout, "{}", termion::cursor::Goto((size+6) as u16,(current_line+1) as u16)).unwrap();
                    buffer = lines[current_line].text.clone();
                }
                Key::Char('q') => break,
                Key::Char(c) => {
                    buffer.push(c);
                    write!(stdout, "{}", c).unwrap();
                }
                Key::Backspace => {
                    let (x, _) = stdout.cursor_pos().unwrap();
                    if x <= 6 {
                        if current_line == 0 {
                            continue;
                        }
                        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

                        let size = lines[current_line - 1].text.chars().count();
                        if size != 0 {
                            write!(
                                stdout,
                                "{}",
                                termion::cursor::Right(size.try_into().unwrap())
                                )
                                .unwrap();
                            buffer = lines[current_line - 1].text.clone();
                        }

                        write!(stdout, "{}", termion::cursor::Up(1)).unwrap();
                        current_line -= 1;
                    } else {
                        buffer.pop();
                        write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
                        write!(stdout, "{}", " ").unwrap();
                        write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
                    }
                }
                _ => {}
            }
        }
        stdout.flush().unwrap();
    }
}
