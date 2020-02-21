extern crate chrono;
extern crate clap;
extern crate rand;
extern crate termion;
use chrono::Local;
mod drawing;
use clap::{App, Arg};
use drawing::{Blank, Draw, Frame, Label, Numbers};
use rand::{distributions::Uniform, prelude::*};
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::{thread, time};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::{async_stdin, input::TermRead};
use termion::{clear, color, cursor};

// for Colossal: 104
// for Doom: 75
const LENGTH_OF_CLOCK: u16 = 75;

fn main() -> std::io::Result<()> {
    static COLORS_AVAILABLE: &str = "Colors available: red, green, blue, yellow, magenta";

    let command_args = App::new("rmatrix")
        .version("0.9")
        .author("Ian W. Github/twitter: <&iwahbe>")
        .about(
            "Rust implementation of cmatrix\n\
                Full credit to the original authors.",
        )
        .arg(
            Arg::with_name("main_color")
                .short("m")
                .help(COLORS_AVAILABLE)
                .long("maincolor")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("end_color")
                .short("e")
                .help(COLORS_AVAILABLE)
                .long("end")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("reverse")
                .short("r")
                .help("Reverses direction")
                .long("reverse")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("horizontal")
                .short("h")
                .help("Changes orientation")
                .long("horizontal")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("clock")
                .short("c")
                .long("clock")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("fortune")
                .long("fortune")
                .short("f")
                .help("Displays fortune quotes"),
        )
        .get_matches();
    // variable setup
    let end_color: &dyn color::Color = match &command_args.args.get("end_color") {
        Some(a) => match get_color(a.vals[0].clone()) {
            Some(c) => c,
            None => {
                println!("Bad end color, see --help");
                return Ok(());
            }
        },
        None => &color::White,
    };
    let main_color: &dyn color::Color = match &command_args.args.get("main_color") {
        Some(a) => match get_color(a.vals[0].clone()) {
            Some(c) => c,
            None => {
                println!("Bad main color, see --help");
                return Ok(());
            }
        },
        None => &color::Blue,
    };

    let reverse = command_args.args.get("reverse").is_some();
    let horizontal = command_args.args.get("horizontal").is_some();
    let clock = command_args.args.get("clock").is_some();
    let fortune = command_args.args.get("fortune").is_some();
    // main loop
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().keys();

    loop {
        match hot_loop(
            &mut stdin,
            &mut stdout,
            &main_color,
            &end_color,
            reverse,
            horizontal,
            clock,
            fortune,
        )? {
            ExitReason::Quite => break,
            ExitReason::SizeChange => thread::sleep(time::Duration::from_secs_f32(0.1)),
        }
    }

    // cleanup, needed to return to normal state
    write!(
        stdout,
        "{goto}{clear}{show_cursor}",
        clear = clear::All,
        goto = cursor::Goto(1, 1),
        show_cursor = cursor::Show
    )?;
    Ok(())
}

enum ExitReason {
    Quite,
    SizeChange,
}

fn hot_loop(
    mut stdin: &mut termion::input::Keys<termion::AsyncReader>,
    mut stdout: &mut dyn Write,
    main_color: &dyn color::Color,
    second_color: &dyn color::Color,
    reverse: bool,
    horizontal: bool,
    clock: bool,
    fortune: bool,
) -> std::io::Result<ExitReason> {
    let (x_size, y_size) = termion::terminal_size()?;
    write!(stdout, "{}{}", cursor::Hide, clear::All)?;
    let mut columns: Vec<Column> = if !horizontal {
        1..x_size
    } else {
        0..y_size + 1
    }
    .into_iter()
    .map(|c| {
        Column::new(
            c,
            if !horizontal { y_size } else { x_size },
            reverse,
            horizontal,
        )
    })
    .collect();
    let mut loop_time = 0;
    let mut quote: Option<String> = if fortune {
        Some("foo".to_owned())
    } else {
        None
    };
    // main loop initialize
    write!(stdout, "{}{}", cursor::Hide, clear::All)?;
    // main loop
    loop {
        if termion::terminal_size()? != (x_size, y_size) {
            return Ok(ExitReason::SizeChange);
        }
        for c in &mut stdin {
            match c.unwrap() {
                Key::Char('q') => return Ok(ExitReason::Quite),
                _ => {}
            }
        }
        let mut forbidden = HashSet::new();
        if clock {
            let t = format!("{}", Local::now().format("%I:%M:%S%p"));
            let now_clock = Frame::from(
                '#',
                Box::new(Frame::from(
                    ' ',
                    Box::new(Numbers::from(&t).with_min_width(LENGTH_OF_CLOCK)),
                )),
            );
            let clock_size = now_clock.size();
            forbidden.extend(now_clock.draw(&mut stdout, 10, y_size - clock_size.height() - 4)?);
        }
        if let Some(q) = quote.as_mut() {
            const Y_DOWN: u16 = 5;
            if loop_time == 0 {
                let quote = std::process::Command::new("fortune").output()?;
                if quote.status.success() {
                    let old = Blank::from(
                        Frame::from('#', Box::new(Frame::from(' ', Box::new(Label::from(&q)))))
                            .size(),
                    );
                    forbidden.extend(old.draw(
                        &mut stdout,
                        x_size - old.size().width() - 10,
                        Y_DOWN,
                    )?);
                    *q = String::from_utf8_lossy(&quote.stdout).to_string();
                }
                let new = Blank::from(
                    Frame::from('#', Box::new(Frame::from(' ', Box::new(Label::from(&q))))).size(),
                );
                forbidden.extend(new.draw(
                    &mut stdout,
                    x_size - new.size().width() - 10,
                    Y_DOWN,
                )?);
            } else {
                let label = Frame::from('#', Box::new(Frame::from(' ', Box::new(Label::from(&q)))));
                forbidden.extend(label.draw(
                    &mut stdout,
                    x_size - label.size().width() - 10,
                    Y_DOWN,
                )?);
            }
        }

        stdout.flush()?;
        thread::sleep(time::Duration::from_secs_f32(
            if horizontal { 0.5 } else { 1.0 } * 0.05,
        ));
        loop_time += 1;
        loop_time = loop_time % 100;

        for c in &mut columns {
            c.update(&mut stdout, main_color, second_color, &forbidden)?;
        }
    }
}

struct Column {
    start: u16,
    end: u16,
    max_height: u16,
    column: u16,
    last_made: char,
    delay: u16,
    reverse: bool,
    horizontal: bool,
}

impl Column {
    fn new(column: u16, max_height: u16, rev: bool, horizontal: bool) -> Self {
        Self {
            start: if !rev {
                0 + horizontal as u16
            } else {
                max_height
            },
            end: if !rev {
                0 + horizontal as u16
            } else {
                max_height
            },
            max_height: max_height,
            column: column,
            last_made: ' ',
            delay: Uniform::new_inclusive(0, if !horizontal { 150 } else { 300 })
                .sample(&mut rand::thread_rng()),
            reverse: rev,
            horizontal: horizontal,
        }
    }
    fn update<T: Write>(
        &mut self,
        writer: &mut T,
        c1: &dyn color::Color,
        c2: &dyn color::Color,
        forbidden: &HashSet<(u16, u16)>,
    ) -> std::io::Result<()> {
        if self.delay == 0 {
            let action = Uniform::new_inclusive(0, 2).sample(&mut rand::thread_rng());
            let lowest = if !self.horizontal { 0 } else { 1 };
            if (!self.reverse && self.max_height != self.end && self.max_height != self.start)
                || (self.reverse && lowest != self.end && lowest != self.start)
            {
                if action == 1 || action == 2 {
                    self.add_last_char(writer, c1, c2, forbidden)?;
                }
                if action == 2 || action == 3 {
                    self.delete_first_char(writer, forbidden)?;
                }
            } else if ((!self.reverse && self.max_height == self.end)
                || (self.reverse && lowest == self.end))
                && self.end == self.start
            {
                // finished column
                let new = if !self.reverse {
                    lowest
                } else {
                    self.max_height
                };
                self.start = new;
                self.end = new;
                self.delay = Uniform::new_inclusive(0, 100).sample(&mut rand::thread_rng());
            } else if (!self.reverse && self.max_height == self.end)
                || (self.reverse && lowest == self.end)
            {
                // finishing up column
                self.delete_first_char(writer, forbidden)?;
                self.fix_last_char(writer, c1, forbidden)?;
            }
        } else {
            self.delay -= 1;
            self.delete_last_char(writer, forbidden)?;
        }

        Ok(())
    }

    fn delete_first_char<T: Write>(
        &mut self,
        writer: &mut T,
        forbidden: &HashSet<(u16, u16)>,
    ) -> std::io::Result<()> {
        // delete last char created
        let pair = if !self.horizontal {
            (self.column, self.start)
        } else {
            (self.start, self.column)
        };
        if forbidden.contains(&pair) {
        } else {
            write!(
                writer,
                "{goto}{space}",
                goto = cursor::Goto(pair.0, pair.1),
                space = " "
            )?;
        }
        if !self.reverse {
            self.start += 1;
        } else {
            self.start -= 1;
        };
        Ok(())
    }

    fn delete_last_char<T: Write>(
        &mut self,
        writer: &mut T,
        forbidden: &HashSet<(u16, u16)>,
    ) -> std::io::Result<()> {
        let lowest = if self.horizontal { 1 } else { 0 };
        let pair = if !self.horizontal {
            (
                self.column,
                if !self.reverse {
                    self.max_height
                } else {
                    lowest
                },
            )
        } else {
            (
                if !self.reverse {
                    self.max_height
                } else {
                    lowest
                },
                self.column,
            )
        };
        if forbidden.contains(&pair) {
        } else {
            write!(writer, "{goto} ", goto = cursor::Goto(pair.0, pair.1))?;
        }
        Ok(())
    }

    fn add_last_char<T: Write>(
        &mut self,
        writer: &mut T,
        c1: &dyn color::Color,
        c2: &dyn color::Color,
        forbidden: &HashSet<(u16, u16)>,
    ) -> std::io::Result<()> {
        // fix color of old char
        self.fix_last_char(writer, c1, forbidden)?;
        if self.reverse == false {
            self.end += 1;
        } else {
            self.end -= 1;
        };
        let pair = if !self.horizontal {
            (self.column, self.end)
        } else {
            (self.end, self.column)
        };
        if forbidden.contains(&pair) {
        } else {
            // create new char at end
            self.last_made = random_char();
            write!(
                writer,
                "{color}{goto}{rand}{reset}",
                color = color::Fg(c2),
                goto = cursor::Goto(pair.0, pair.1),
                rand = self.last_made,
                reset = color::Fg(color::Reset),
            )?;
        }
        Ok(())
    }

    fn fix_last_char<T: Write>(
        &mut self,
        writer: &mut T,
        c1: &dyn color::Color,
        forbidden: &HashSet<(u16, u16)>,
    ) -> std::io::Result<()> {
        let pair = if !self.horizontal {
            (self.column, self.end)
        } else {
            (self.end, self.column)
        };
        if forbidden.contains(&pair) {
        } else {
            write!(
                writer,
                "{color}{goto}{c}{reset}",
                color = color::Fg(c1),
                goto = cursor::Goto(pair.0, pair.1),
                c = self.last_made,
                reset = color::Fg(color::Reset),
            )?;
        }
        Ok(())
    }
}

fn random_char() -> char {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789)(*&^%$#@!~";
    let idx = rand::thread_rng().gen_range(0, CHARSET.len());
    CHARSET[idx] as char
}

fn get_color(color: std::ffi::OsString) -> Option<&'static dyn color::Color> {
    let s = match color.into_string() {
        Ok(s) => s,
        Err(_) => return None,
    };
    match s.as_str() {
        "red" => Some(&color::Red),
        "green" => Some(&color::Green),
        "blue" => Some(&color::Blue),
        "yellow" => Some(&color::Yellow),
        "magenta" => Some(&color::Magenta),
        _ => None,
    }
}
