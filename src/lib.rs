#[allow(unused_variables)]

use std::cmp::PartialOrd;

pub mod math;

pub struct Context {
    value: i32
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub enum Winding {
    CounterClockwise,
    Clockwise,
}

pub enum Solidity {
    Solid = 10,
    Empty = 20,
}

pub enum LineCap {
    Butt,
    Round,
    Square,
    Bevel,
    Miter,
}

pub enum Align {
    Left = 1 << 0,
    Center = 1 << 1,
    Right = 1 << 2,
    Top = 1 << 3,
    Middle = 1 << 4,
    Bottom = 1 << 5,
    Baseline = 1 << 6,
}

struct GlyphPosition {
    // position of the glyph in the string
    offset: i32,
    // the x coordinate of the logical glyph position
    x: f32,
    // bounds of the glyph shape
    minx: f32,
    miny: f32,
}

impl Color {
    /// Creates a new color from 32 bits representing the 4 channels.
    pub fn hex(rgb: i32) -> Color {
        Color {
            r: ((rgb >> 16) & 0xff) as f32 / 255.0,
            g: ((rgb >> 8) & 0xff) as f32 / 255.0,
            b: (rgb & 0xff) as f32 / 255.0,
            a: 1.0
        }
    }

    /// Creates a new color from 32 bits representing the 4 channels and an alpha value.
    pub fn hexa(rgb: i32, alpha: f32) -> Color {
        Color {
            r: ((rgb >> 16) & 0xff) as f32 / 255.0,
            g: ((rgb >> 8) & 0xff) as f32 / 255.0,
            b: (rgb & 0xff) as f32 / 255.0,
            a: alpha
        }
    }

    /// Creates a color from the specified red, green, and blue channel values.
    pub fn rgb(red: f32, green: f32, blue: f32) -> Color {
        Color::rgba(red, green, blue, 1.0)
    }

    /// Creates a color from the specified red, green, blue and alpha channel values.
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: alpha
        }
    }

    /// Returns color value specified by hue, saturation and lightness.
    pub fn hsl(h: f32, s: f32, l: f32) {
        Color::hsla(h, s, l, 1.0)
    }

    /// Returns color value specified by hue, saturation, lightness and alpha.
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) {
        let mut m1: f32;
        let mut m2: f32;
        let mut col: Color;

        // TODO: implement
    }

    /// Returns a color linearly interpolated between `from` and `to`.
    pub fn lerp(from: Color, to: Color, u: f32) {
    }

    /// Returns a copy of this color with the requested `alpha` value.
    pub fn withAlpha(self, alpha: f32) -> Color {
        return Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha
        }
    }

    fn hue_helper(h: f32, m1: f32, m2: f32) -> f32 {
        let hc = math::clamp(h, 0.0, 1.0);

        if hc < 1.0 / 6.0 {
            return m1 + (m2 - m1) * hc * 6.0;
        }
        if hc < 4.0 / 6.0 {
            return m1 + (m2 - m1) * (2.0 / 3.0 - hc) * 6.0;
        }
        return m1
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            value: 0
        }
    }

    pub fn begin_frame(&self, window: (i32, i32), dpi: f32) {
    }

    pub fn end_frame(&self) {
    }

    pub fn report_value(self) {
        println!("value: {}", self.value)
    }
}