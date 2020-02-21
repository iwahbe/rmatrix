use std::collections::HashSet;
use std::io::Write;
use termion::cursor;

type Digit = fonts::Doom;
use fonts::Font;

pub struct Numbers {
    digits: String,
    size: Size,
}

impl Numbers {
    pub fn from(digits: &str) -> Self {
        let mut out = Self::new();
        out.setup(digits);
        out
    }
    pub fn with_min_width(mut self, width: u16) -> Self {
        if self.size.width < width {
            self.size.width = width;
        }
        self
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
    fn setup(&mut self, new: &str) {
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
            .0;
            w += size.width;
            h = size.height;
        }
        self.digits = new.to_owned();
        self.size.width = w + new.len() as u16;
        self.size.height = h - 1;
    }
}

#[derive(Copy, Clone)]
pub struct Size {
    height: u16,
    width: u16,
}

impl Size {
    pub fn height(&self) -> u16 { self.height }
    pub fn width(&self) -> u16 { self.width }
    fn from(h: u16, w: u16) -> Self {
        Self {
            height: h,
            width: w,
        }
    }
}

pub trait Draw {
    fn size(&self) -> Size;
    fn draw(&self, writer: &mut dyn Write, x: u16, y: u16) -> std::io::Result<HashSet<(u16, u16)>>;
}

impl Draw for Numbers {
    fn size(&self) -> Size { self.size }
    fn draw(&self, writer: &mut dyn Write, x: u16, y: u16) -> std::io::Result<HashSet<(u16, u16)>> {
        let mut taken = HashSet::new();
        for row in 0..(Digit::one().1).inner().len() {
            write!(writer, "{}", cursor::Goto(x, y + row as u16))?;
            let mut index = 0;
            // for better padding
            write!(writer, " ")?;
            taken.insert((x + index, y + row as u16));
            index += 1;
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
                .1)
                    .inner()[row];
                for _ in 0..chrs.len() {
                    taken.insert((x + index, y + row as u16));
                    index += 1;
                }
                write!(writer, "{}", chrs)?;

                write!(writer, " ")?;
                taken.insert((x + index, y + row as u16));
                index += 1;
            }
            let fix_index = index;
            for _ in 0..(self.size.width - fix_index as u16) {
                write!(writer, " ")?;
                taken.insert((x + index, y + row as u16));
                index += 1;
            }
        }
        Ok(taken)
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
    fn draw(&self, writer: &mut dyn Write, x: u16, y: u16) -> std::io::Result<HashSet<(u16, u16)>> {
        let mut taken = HashSet::new();
        let size = self.size();
        write!(writer, "{}", cursor::Goto(x, y))?;
        for i in 0..size.width - 2 {
            write!(writer, "{}", self.material)?;
            taken.insert((x + i, y));
        }
        for y0 in y..y + size.height {
            write!(
                writer,
                "{g1}{m}{g2}{m}",
                g1 = cursor::Goto(x, y0),
                g2 = cursor::Goto(x + size.width - 2, y0),
                m = self.material
            )?;
            taken.insert((x, y0));
            taken.insert((x + size.width - 2, y0));
        }
        write!(writer, "{}", cursor::Goto(x, y + size.height))?;
        for i in 0..self.size().width - 1 {
            write!(writer, "{}", self.material)?;
            taken.insert((x + i, y + size.height));
        }
        let inner_hitbox = self.inner.draw(writer, x + 1, y + 1)?;
        taken.extend(inner_hitbox);
        Ok(taken)
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

pub struct Label {
    lines: Vec<String>,
    size: Size,
}

impl Label {
    pub fn from(contents: &str) -> Self {
        let mut size = Size {
            width: 0,
            height: 0,
        };
        let lines = contents
            .lines()
            .map(|x| {
                size.height += 1;
                if size.width < x.len() as u16 + 1 {
                    size.width = x.len() as u16 + 1;
                }

                x.to_owned()
            })
            .collect();
        size.height -= 1;
        Self {
            lines: lines,
            size: size,
        }
    }
}

impl Draw for Label {
    fn size(&self) -> Size { self.size }
    fn draw(&self, writer: &mut dyn Write, x: u16, y: u16) -> std::io::Result<HashSet<(u16, u16)>> {
        for i in 0..self.lines.len() {
            write!(
                writer,
                "{goto}{line}",
                goto = cursor::Goto(x, y + i as u16),
                line = self.lines[i]
            )?;
        }
        Ok(dense_hitbox(x, self.size.width(), y, self.size.height()))
    }
}

fn dense_hitbox(x0: u16, x_len: u16, y0: u16, y_len: u16) -> HashSet<(u16, u16)> {
    let mut taken = HashSet::<(u16, u16)>::new();
    for x in x0..x0 + x_len {
        for y in y0..y0 + y_len + 1 {
            taken.insert((x, y));
        }
    }
    taken
}

pub struct Blank {
    size: Size,
}

impl Blank {
    pub fn from(mut size: Size) -> Self {
        size.height += 1;
        Self { size: size }
    }
}

impl Draw for Blank {
    fn size(&self) -> Size { self.size }
    fn draw(&self, writer: &mut dyn Write, x: u16, y: u16) -> std::io::Result<HashSet<(u16, u16)>> {
        for y0 in y..y + self.size.height() {
            write!(writer, "{}", cursor::Goto(x, y0))?;
            for _ in x..x + self.size.width() {
                write!(writer, " ")?;
            }
        }
        Ok(dense_hitbox(x, self.size.width(), y, self.size.height()))
    }
}

mod fonts {

    use super::Size;
    pub trait Font {
        fn one() -> (Size, Self);
        fn two() -> (Size, Self);
        fn three() -> (Size, Self);
        fn four() -> (Size, Self);
        fn five() -> (Size, Self);
        fn six() -> (Size, Self);
        fn seven() -> (Size, Self);
        fn eight() -> (Size, Self);
        fn nine() -> (Size, Self);
        fn zero() -> (Size, Self);
        fn a() -> (Size, Self);
        fn p() -> (Size, Self);
        fn m() -> (Size, Self);
        fn colon() -> (Size, Self);
        fn space() -> (Size, Self);
        fn inner(&self) -> &Vec<&'static str>;
        fn clock_size(&self) -> u16;
    }

    pub struct Doom(Vec<&'static str>);
    impl Font for Doom {
        fn inner(&self) -> &Vec<&'static str> { &self.0 }
        fn one() -> (Size, Self) {
            (
                Size::from(6, 6),
                Self(vec![
                    r" __  ", r"/  | ", r"`| | ", r" | | ", r"_| |_", r"\___/",
                ]),
            )
        }
        fn two() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r" _____ ", r"/ __  \", r"`' / /'", r"  / /  ", r"./ /___", r"\_____/",
                ]),
            )
        }
        fn three() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r" _____ ", r"|____ |", r"    / /", r"    \ \", r".___/ /", r"\____/ ",
                ]),
            )
        }
        fn four() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r"   ___ ", r"  /   |", r" / /| |", r"/ /_| |", r"\___  |", r"    |_/",
                ]),
            )
        }
        fn five() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r" _____ ", r"|  ___|", r"|___ \ ", r"    \ \", r"/\__/ /", r"\____/ ",
                ]),
            )
        }
        fn six() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r"  ____ ", r" / ___|", r"/ /___ ", r"| ___ \", r"| \_/ |", r"\_____/",
                ]),
            )
        }
        fn seven() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r" ______", r"|___  /", r"   / / ", r"  / /  ", r"./ /   ", r"\_/    ",
                ]),
            )
        }
        fn eight() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r" _____ ", r"|  _  |", r" \ V / ", r" / _ \ ", r"| |_| |", r"\_____/",
                ]),
            )
        }
        fn nine() -> (Size, Self) {
            (
                Size::from(6, 6),
                Self(vec![
                    r" _____ ", r"|  _  |", r"| |_| |", r"\____ |", r".___/ /", r"\____/ ",
                ]),
            )
        }
        fn zero() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r" _____ ", r"|  _  |", r"| |/' |", r"|  /| |", r"\ |_/ /", r" \___/ ",
                ]),
            )
        }
        fn colon() -> (Size, Self) {
            (
                Size::from(6, 3),
                Self(vec![r"   ", r" _ ", r"(_)", r"   ", r" _ ", r"(_)"]),
            )
        }
        fn a() -> (Size, Self) {
            (
                Size::from(6, 7),
                Self(vec![
                    r"  ___  ", r" / _ \ ", r"/ /_\ \", r"|  _  |", r"| | | |", r"\_| |_/",
                ]),
            )
        }
        fn p() -> (Size, Self) {
            (
                Size::from(6, 8),
                Self(vec![
                    r"______ ", r"| ___ \", r"| |_/ /", r"|  __/ ", r"| |    ", r"\_|    ",
                ]),
            )
        }
        fn m() -> (Size, Self) {
            (
                Size::from(6, 8),
                Self(vec![
                    r"___  ___",
                    r"|  \/  |",
                    r"| .  . |",
                    r"| |\/| |",
                    r"| |  | |",
                    r"\_|  |_/",
                ]),
            )
        }
        fn space() -> (Size, Self) {
            (
                Size::from(6, 1),
                Self(vec![r" ", r" ", r" ", r" ", r" ", r" "]),
            )
        }
        fn clock_size(&self) -> u16 { 75 }
    }

    pub struct Colossal(Vec<&'static str>);
    impl Font for Colossal {
        fn inner(&self) -> &Vec<&'static str> { &self.0 }
        fn clock_size(&self) -> u16 { 104 }
        // Colossal font: "https://onlineasciitools.com/convert-text-to-ascii-art"
        fn one() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 8,
                },
                Self(vec![
                    " d888  ", "d8888  ", "  888  ", "  888  ", "  888  ", "  888  ", "  888  ",
                    "8888888",
                ]),
            )
        }
        fn two() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 11,
                },
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
            )
        }
        fn three() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 10,
                },
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
            )
        }
        fn four() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 10,
                },
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
            )
        }
        fn five() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 10,
                },
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
            )
        }
        fn six() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 10,
                },
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
            )
        }
        fn seven() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 10,
                },
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
            )
        }
        fn eight() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 11,
                },
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
            )
        }
        fn nine() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 10,
                },
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
            )
        }
        fn zero() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 11,
                },
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
            )
        }
        fn colon() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 3,
                },
                Self(vec!["   ", "d8b", "Y8P", "   ", "   ", "d8b", "Y8P", "   "]),
            )
        }
        fn a() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 12,
                },
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
            )
        }
        fn p() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 10,
                },
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
            )
        }
        fn m() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 13,
                },
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
            )
        }
        fn space() -> (Size, Self) {
            (
                Size {
                    height: 8,
                    width: 2,
                },
                Self(vec!["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "]),
            )
        }
    }
}
