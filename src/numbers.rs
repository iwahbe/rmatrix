struct Digit(Vec<&'static str>);
use std::io::Write;
use termion::cursor;

impl Digit {
    // Colossal font: "https://onlineasciitools.com/convert-text-to-ascii-art"
    fn one() -> (Self, Size) {
        (
            Self(vec![
                " d888  ", "d8888  ", "  888  ", "  888  ", "  888  ", "  888  ", "  888  ",
                "8888888",
            ]),
            Size {
                height: 8,
                width: 8,
            },
        )
    }
    fn two() -> (Self, Size) {
        (
            Self(vec![
                " .d8888b.  ",
                "d88P  Y88b ",
                "       888 ",
                "     .d88P ",
                " .od888P\"  ",
                "d88P\"      ",
                "888\"       ",
                "888888888  ",
            ]),
            Size {
                height: 8,
                width: 11,
            },
        )
    }
    fn three() -> (Self, Size) {
        (
            Self(vec![
                " .d8888b. ",
                "d88P  Y88b",
                "     .d88P",
                "    8888\" ",
                "     \"Y8b.",
                "888    888",
                "Y88b  d88P",
                " \"Y8888P\" ",
            ]),
            Size {
                height: 8,
                width: 10,
            },
        )
    }
    fn four() -> (Self, Size) {
        (
            Self(vec![
                "    d8888 ",
                "   d8P888 ",
                "  d8P 888 ",
                " d8P  888 ",
                "d88   888 ",
                "8888888888",
                "      888 ",
                "      888 ",
            ]),
            Size {
                height: 8,
                width: 10,
            },
        )
    }
    fn five() -> (Self, Size) {
        (
            Self(vec![
                "888888888 ",
                "888       ",
                "888       ",
                "8888888b. ",
                "     \"Y88b",
                "       888",
                "Y88b  d88P",
                " \"Y8888P\" ",
            ]),
            Size {
                height: 8,
                width: 10,
            },
        )
    }
    fn six() -> (Self, Size) {
        (
            Self(vec![
                " .d8888b. ",
                "d88P  Y88b",
                "888       ",
                "888d888b. ",
                "888P \"Y88b",
                "888    888",
                "Y88b  d88P",
                " \"Y8888P\" ",
            ]),
            Size {
                height: 8,
                width: 10,
            },
        )
    }
    fn seven() -> (Self, Size) {
        (
            Self(vec![
                "8888888888",
                "      d88P",
                "     d88P ",
                "    d88P  ",
                " 88888888 ",
                "  d88P    ",
                " d88P     ",
                "d88P      ",
            ]),
            Size {
                height: 8,
                width: 10,
            },
        )
    }
    fn eight() -> (Self, Size) {
        (
            Self(vec![
                " .d8888b.  ",
                "d88P  Y88b ",
                "Y88b. d88P ",
                " \"Y88888\"  ",
                ".d8P\"\"Y8b. ",
                "888    888 ",
                "Y88b  d88P ",
                " \"Y8888P\"  ",
            ]),
            Size {
                height: 8,
                width: 11,
            },
        )
    }
    fn nine() -> (Self, Size) {
        (
            Self(vec![
                " .d8888b. ",
                "d88P  Y88b",
                "888    888",
                "Y88b. d888",
                " \"Y888P888",
                "       888",
                "Y88b  d88P",
                " \"Y8888P\" ",
            ]),
            Size {
                height: 8,
                width: 10,
            },
        )
    }
    fn zero() -> (Self, Size) {
        (
            Self(vec![
                " .d8888b. ",
                "d88P  Y88b",
                "888    888",
                "888    888",
                "888    888",
                "888    888",
                "Y88b  d88P",
                " \"Y8888P\" ",
            ]),
            Size {
                height: 8,
                width: 11,
            },
        )
    }
    fn colon() -> (Self, Size) {
        (
            Self(vec!["   ", "d8b", "Y8P", "   ", "   ", "d8b", "Y8P", "   "]),
            Size {
                height: 8,
                width: 3,
            },
        )
    }
    fn a() -> (Self, Size) {
        (
            Self(vec![
                "       d8888",
                "      d88888",
                "     d88P888",
                "    d88P 888",
                "   d88P  888",
                "  d88P   888",
                " d8888888888",
                "d88P     888",
            ]),
            Size {
                height: 8,
                width: 12,
            },
        )
    }
    fn p() -> (Self, Size) {
        (
            Self(vec![
                "8888888b. ",
                "888   Y88b",
                "888    888",
                "888   d88P",
                "8888888P\" ",
                "888       ",
                "888       ",
                "888       ",
            ]),
            Size {
                height: 8,
                width: 10,
            },
        )
    }
    fn m() -> (Self, Size) {
        (
            Self(vec![
                "888b     d888",
                "8888b   d8888",
                "88888b.d88888",
                "888Y88888P888",
                "888 Y888P 888",
                "888  Y8P  888",
                "888   \"   888",
                "888       888",
            ]),
            Size {
                height: 8,
                width: 13,
            },
        )
    }
    fn space() -> (Self, Size) {
        (
            Self(vec!["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "]),
            Size {
                height: 8,
                width: 2,
            },
        )
    }
}

pub struct Numbers {
    digits: String,
    size: Size,
}

impl Numbers {
    pub fn from(digits: &str) -> Self {
        let mut out = Self::new();
        out.update(digits);
        out
    }
    fn new() -> Self {
        Self {
            digits: String::new(),
            size: Size {
                height: 0,
                width: 0,
            },
        }
    }
    fn update(&mut self, new: &str) {
        let mut w = 0;
        let mut h = 0;
        for digit in new.chars() {
            let size = match digit {
                '1' => Digit::one(),
                '2' => Digit::two(),
                '3' => Digit::three(),
                '4' => Digit::four(),
                '5' => Digit::five(),
                '6' => Digit::six(),
                '7' => Digit::seven(),
                '8' => Digit::eight(),
                '9' => Digit::nine(),
                '0' => Digit::zero(),
                ':' => Digit::colon(),
                'P' => Digit::p(),
                'A' => Digit::a(),
                'M' => Digit::m(),
                ' ' => Digit::space(),
                _ => panic!("Bad Numbers input"),
            }
            .1;
            w += size.width;
            h = size.height;
        }
        self.digits = new.to_owned();
        self.size.width = w + new.len() as u16 - 2;
        self.size.height = h - 1;
    }
}

#[derive(Copy, Clone)]
pub struct Size {
    height: u16,
    width: u16,
}

pub trait Draw {
    fn size(&self) -> Size;
    fn draw(&self, writer: &mut dyn Write, x: u16, y: u16) -> std::io::Result<()>;
}

impl Draw for Numbers {
    fn size(&self) -> Size { self.size }
    fn draw(&self, writer: &mut dyn Write, x: u16, y: u16) -> std::io::Result<()> {
        for row in 0..(Digit::one().0).0.len() {
            write!(writer, "{}", cursor::Goto(x, y + row as u16))?;
            for digit in self.digits.char_indices() {
                let chrs = (match digit.1 {
                    '1' => Digit::one(),
                    '2' => Digit::two(),
                    '3' => Digit::three(),
                    '4' => Digit::four(),
                    '5' => Digit::five(),
                    '6' => Digit::six(),
                    '7' => Digit::seven(),
                    '8' => Digit::eight(),
                    '9' => Digit::nine(),
                    '0' => Digit::zero(),
                    ':' => Digit::colon(),
                    'P' => Digit::p(),
                    'A' => Digit::a(),
                    'M' => Digit::m(),
                    ' ' => Digit::space(),
                    _ => panic!("Bad Number input"),
                }
                .0)
                    .0[row];
                write!(writer, "{}", chrs)?;
                if digit.0 != self.digits.len() - 1 {
                    write!(writer, " ")?;
                }
            }
        }
        Ok(())
    }
}

pub struct Frame {
    inner: Box<dyn Draw>,
    material: char,
}

impl Draw for Frame {
    fn size(&self) -> Size {
        let inner = self.inner.size();
        Size {
            height: inner.height + 2,
            width: inner.width + 2,
        }
    }
    fn draw(&self, writer: &mut dyn Write, x: u16, y: u16) -> std::io::Result<()> {
        write!(writer, "{}", cursor::Goto(x, y))?;
        let size = self.size();
        for _ in 0..size.width {
            write!(writer, "{}", self.material)?;
        }
        for y0 in y..y + size.height + 1 {
            write!(
                writer,
                "{g1}{m}{g2}{m}",
                g1 = cursor::Goto(x, y0),
                g2 = cursor::Goto(x + size.width, y0),
                m = self.material
            )?;
        }
        write!(writer, "{}", cursor::Goto(x, y + size.height))?;
        for _ in 0..self.size().width {
            write!(writer, "{}", self.material)?;
        }
        self.inner.draw(writer, x + 1, y + 1)?;
        Ok(())
    }
}

impl Frame {
    pub fn from(material: char, contents: Box<dyn Draw>) -> Self {
        Self {
            inner: contents,
            material: material,
        }
    }
}
