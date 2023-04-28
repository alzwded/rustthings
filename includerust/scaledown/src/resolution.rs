use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    combinator::map_res,
};
use std::fmt;
use std::cmp::{PartialOrd, Ordering,};

/// cbindgen:field-names=[w,h]
/// cbindgen:derive-eq
/// cbindgen:derive-neq
/// cbindgen:derive-constructor
#[repr(C)]
#[derive(Default,Debug)]
pub struct Resolution {
    w: u32,
    h: u32
}

fn n_from_s(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 10)
}

fn parse_resolution(input: &str) -> IResult<&str, Resolution> {
    let (input, w) = map_res(
        take_while1(|c: char| c.is_digit(10)),
        n_from_s
    )(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, h) = map_res(
        take_while1(|c: char| c.is_digit(10)),
        n_from_s
    )(input)?;
    Ok((input, Resolution {w, h}))
}

impl Resolution {
    pub fn from_string(s: &String) -> Self {
        match parse_resolution(s) {
            Ok((_, r)) => return r,
            Err(e) => panic!("{}", e),
        }
    }
    pub fn new(w: u32, h: u32) -> Self {
        Resolution { w, h }
    }

    pub fn width(&self) -> u32 {
        self.w
    }
    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn scale_to_square(&self, to: u32) -> Self {
        let max = std::cmp::max(self.w, self.h) as f32;
        let f = to as f32 / max;
        Resolution { w: (self.w as f32 * f) as u32, h: (self.h as f32 * f) as u32, }
    }

    pub fn scale_to(&self, target: &Resolution) -> Self {
        /*
        let f;
        {
            let mut q = vec![target.width() as f32 / self.w as f32, target.height() as f32 / self.h as f32];
            q.sort_by(|a, b| a.total_cmp(&b));
            f = q[0];
        }
        */
        let f = (target.width() as f32 / self.w as f32).min(target.height() as f32 / self.h as f32);
        Resolution { w: (self.w as f32 * f) as u32, h: (self.h as f32 * f) as u32, }
    }

    pub fn count(&self) -> usize {
        ( self.width() * self.height() ) as usize
    }
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.w, self.h)
    }
}

impl std::cmp::PartialOrd for Resolution {
    fn partial_cmp(&self, other: &Resolution) -> Option<Ordering> {
        if self.w == other.w && self.h == other.w { return Some(Ordering::Equal); }
        if self.w < other.w && self.h < other.h   { return Some(Ordering::Less); }
        if self.w > other.w && self.h > other.h   { return Some(Ordering::Greater); }
        if self.w < other.w && self.h == other.h  { return Some(Ordering::Less); }
        if self.w == other.w && self.h < other.h  { return Some(Ordering::Less); }
        if self.w > other.w && self.h == other.h  { return Some(Ordering::Greater); }
        if self.w == other.w && self.h > other.h  { return Some(Ordering::Greater); }
        None
    }

    fn lt(&self, other: &Resolution) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => true,
            _ => false,
        }
    }

    fn le(&self, other: &Resolution) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => true,
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn gt(&self, other: &Resolution) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            _ => false,
        }
    }

    fn ge(&self, other: &Resolution) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

impl std::cmp::PartialEq for Resolution {
    fn eq(&self, other: &Resolution) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Resolution) -> bool {
        !(self == other)
    }
}
