use clap::{Args, Parser};
use std::fmt::{self, Display};
use std::io::{Error, ErrorKind};
use std::ops::{Add, Sub};
use winsafe::{RECT, SIZE};

#[derive(Debug, Clone, Args)]
pub struct TargetInformation {
    #[arg(short, long, help = "Filter by binary path")]
    pub path_endswith: Vec<String>,
    #[arg(short, long, help = "Filter by title")]
    pub title_contains: Vec<String>,
    #[arg(short, long, value_parser = parse_size, default_value_t = Size::default(), help = "Additional offset for window")]
    pub offset: Size,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(flatten)]
    pub target: TargetInformation,
    #[arg(value_parser = parse_size, help = "Set the window size if it's set.")]
    pub size: Option<Size>,
}

#[derive(Debug, Copy, Clone, Parser)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Add for Size {
    type Output = Size;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<RECT> for Size {
    fn from(v: RECT) -> Self {
        Self {
            x: (v.right - v.left) as usize,
            y: (v.bottom - v.top) as usize,
        }
    }
}

impl From<Size> for SIZE {
    fn from(v: Size) -> Self {
        Self {
            cx: v.x as i32,
            cy: v.y as i32,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

fn parse_size(arg: &str) -> Result<Size, Error> {
    let mut res = arg.split('x');

    let Some(x) = res.next() else {
        return Err(Error::new(ErrorKind::InvalidInput, "Unexpected Input"));
    };

    let Some(y) = res.next() else {
        return Err(Error::new(ErrorKind::InvalidInput, "Unexpected Input"));
    };

    let None = res.next() else {
        return Err(Error::new(ErrorKind::InvalidInput, "Unexpected Input"));
    };

    Ok(Size {
        x: x.parse()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Unexpected Input"))?,
        y: y.parse()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Unexpected Input"))?,
    })
}
