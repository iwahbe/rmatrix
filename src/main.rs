extern crate clap;
extern crate rand;
extern crate termion;
use clap::{App, Arg};
use rand::{distributions::Uniform, prelude::*};
use std::io::{stdout, Write};
use std::{thread, time};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::{async_stdin, input::TermRead};
use termion::{clear, color, cursor};

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
    // main loop
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().keys();

    loop {
        match hot_loop(&mut stdin, &mut stdout, &main_color, &end_color)? {
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
) -> std::io::Result<ExitReason> {
    let (x_size, y_size) = termion::terminal_size()?;
    write!(stdout, "{}{}", cursor::Hide, clear::All)?;
    let mut columns: Vec<Column> = (1..x_size)
        .into_iter()
        .map(|c| Column::new(c, y_size))
        .collect();
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
        stdout.flush()?;
        for c in &mut columns {
            c.update(&mut stdout, main_color, second_color)?;
        }

        thread::sleep(time::Duration::from_secs_f32(0.05));
    }
}

struct Column {
    start: u16,
    end: u16,
    max_height: u16,
    column: u16,
    last_made: char,
    delay: u16,
}

impl Column {
    fn new(column: u16, max_height: u16) -> Self {
        Self {
            start: 0,
            end: 0,
            max_height: max_height,
            column: column,
            last_made: ' ',
            delay: Uniform::new_inclusive(0, 150).sample(&mut rand::thread_rng()),
        }
    }
    fn update<T: Write>(
        &mut self,
        writer: &mut T,
        c1: &dyn color::Color,
        c2: &dyn color::Color,
    ) -> std::io::Result<()> {
        if self.delay == 0 {
            let action = Uniform::new_inclusive(0, 2).sample(&mut rand::thread_rng());
            if self.max_height != self.end && self.max_height != self.start {
                if action == 1 || action == 2 {
                    self.add_last_char(writer, c1, c2)?;
                }
                if action == 2 || action == 3 {
                    self.delete_first_char(writer)?;
                }
            } else if self.max_height == self.end && self.end == self.start {
                // finished column
                self.start = 0;
                self.end = 0;
                self.delay = Uniform::new_inclusive(0, 100).sample(&mut rand::thread_rng());
            } else if self.max_height == self.end {
                // finishing up column
                self.delete_first_char(writer)?;
                self.fix_last_char(writer, c1)?;
            }
        } else {
            self.delay -= 1;
            self.delete_last_char(writer)?;
        }

        Ok(())
    }

    fn delete_first_char<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()> {
        // delete last char created
        write!(
            writer,
            "{goto}{space}",
            goto = cursor::Goto(self.column, self.start),
            space = " "
        )?;
        self.start += 1;
        Ok(())
    }

    fn delete_last_char<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()> {
        write!(
            writer,
            "{goto} ",
            goto = cursor::Goto(self.column, self.max_height),
        )?;
        Ok(())
    }

    fn add_last_char<T: Write>(
        &mut self,
        writer: &mut T,
        c1: &dyn color::Color,
        c2: &dyn color::Color,
    ) -> std::io::Result<()> {
        // fix color of old char
        self.fix_last_char(writer, c1)?;
        self.end += 1;
        // create new char at end
        self.last_made = random_char();
        write!(
            writer,
            "{color}{goto}{rand}{reset}",
            color = color::Fg(c2),
            goto = cursor::Goto(self.column, self.end),
            rand = self.last_made,
            reset = color::Fg(color::Reset),
        )?;
        Ok(())
    }

    fn fix_last_char<T: Write>(
        &mut self,
        writer: &mut T,
        c1: &dyn color::Color,
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{color}{goto}{c}{reset}",
            color = color::Fg(c1),
            goto = cursor::Goto(self.column, self.end),
            c = self.last_made,
            reset = color::Fg(color::Reset),
        )?;
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
