extern crate rand;
extern crate termion;
use rand::{distributions::Uniform, prelude::*};
use std::io::{stdout, Write};
use std::{thread, time};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::{async_stdin, input::TermRead};
use termion::{clear, color, cursor};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide)?;
    let mut stdin = async_stdin().keys();
    let (x_size, y_size) = termion::terminal_size().unwrap();
    let mut columns = get_columns(x_size, 3)
        .into_iter()
        .map(|c| PrintVerticalRandom::new(c, y_size));
    let mut exit = false;
    let mut state: usize = 0;
    while !exit {
        write!(stdout, "{}", clear::All)?;
        for mut c in &mut columns {
            match state {
                0 => c.write(&mut stdout)?,
                1 => c.grow(),
                _ => {}
            }
        }
        state = (state + 1) % 2;
        for c in &mut stdin {
            match c.unwrap() {
                Key::Char('q') => {
                    exit = true;
                    break;
                }
                _ => {}
            }
        }
        stdout.flush()?;
        thread::sleep(time::Duration::from_secs_f32(0.1));
    }
    write!(
        stdout,
        "{goto}{clear}{show_cursor}",
        clear = clear::All,
        goto = cursor::Goto(1, 1),
        show_cursor = cursor::Show
    )?;
    Ok(())
}

fn set_color<T: Write>(writer: &mut T, col: &dyn color::Color) -> std::io::Result<()> {
    write!(writer, "{}", color::Fg(col))?;
    Ok(())
}

fn reset_color<T: Write>(writer: &mut T) -> std::io::Result<()> {
    write!(writer, "{}", color::Fg(color::Reset))?;
    Ok(())
}

struct PrintVerticalRandom {
    string: String,
    height: u16,
    column: u16,
    max_height: u16,
}

impl PrintVerticalRandom {
    fn write<T: Write>(&mut self, writer: &mut T) -> std::io::Result<()> {
        write!(
            writer,
            "{goto}",
            goto = cursor::Goto(self.column, self.height)
        )?;
        let mut len = self.string.len();
        let mut iter = self.string.chars().rev();
        set_color(writer, &color::Red)?;
        while len > 1 {
            write!(
                writer,
                "{c}{back}{down}",
                c = iter.next().unwrap(),
                down = cursor::Down(1),
                back = cursor::Left(1)
            )?;
            len -= 1;
        }
        reset_color(writer)?;
        if let Some(c) = iter.next() {
            write!(writer, "{c}", c = c,)?;
        }
        self.height += 1;
        // if self.height > self.max_height {
        //     self.string = random_char().to_string();
        //     self.height = 1;
        // }
        Ok(())
    }
    fn grow(&mut self) { self.string.push(random_char()); }
    fn new(column: u16, max_height: u16) -> Self {
        let mut s = String::new();
        s.push(random_char());
        Self {
            string: s,
            height: 1,
            column: column,
            max_height: max_height,
        }
    }
}

fn random_char() -> char {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789)(*&^%$#@!~";
    let idx = rand::thread_rng().gen_range(0, CHARSET.len());
    CHARSET[idx] as char
}

fn get_columns(x_size: u16, num_columns: u16) -> std::collections::HashSet<u16> {
    let between = Uniform::new_inclusive(1, x_size);
    let mut r = rand::thread_rng();
    let mut cols = std::collections::HashSet::new();
    while cols.len() < num_columns as usize {
        cols.insert(between.sample(&mut r));
    }
    cols
}
