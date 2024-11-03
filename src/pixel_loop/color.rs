//! Color types and conversion utilities.
//!
//! This module provides RGB and HSL color representations along with conversion
//! functions between different color spaces. It also includes utilities for
//! handling color data as byte slices.

/// An RGBA color representation.
///
/// Each color component (red, green, blue, alpha) is stored as an 8-bit
/// unsigned integer, giving a range of 0-255 for each channel.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    /// Red component [0-255]
    pub r: u8,
    /// Green component [0-255]
    pub g: u8,
    /// Blue component [0-255]
    pub b: u8,
    /// Alpha component [0-255]
    pub a: u8,
}

/// Trait for converting color data to raw bytes.
///
/// This trait enables efficient conversion of color data to byte slices
/// without copying the underlying data.
pub trait ColorAsByteSlice {
    /// Converts the color data to a raw byte slice.
    fn as_byte_slice(&self) -> &[u8];
}

impl ColorAsByteSlice for [Color] {
    fn as_byte_slice(&self) -> &[u8] {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                std::mem::size_of::<Color>() * self.len(),
            )
        };
        byte_slice
    }
}

impl Color {
    /// Creates a new Color from a slice of bytes.
    ///
    /// # Arguments
    /// * `bytes` - Raw byte slice containing RGBA color data
    ///
    /// # Panics
    /// * If the byte slice length is not a multiple of 4
    /// * If the byte slice is not properly aligned for Color struct
    ///
    /// # Examples
    /// ```
    /// use my_crate::Color;
    ///
    /// let bytes = [255, 0, 0, 255, 0, 255, 0, 255];
    /// let colors = Color::from_bytes(&bytes);
    /// assert_eq!(colors.len(), 2);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> &[Self] {
        if bytes.len() % std::mem::size_of::<Color>() != 0 {
            panic!("Color slices can only be initialized with a multiple of 4 byte slices");
        }

        let color_slice = unsafe {
            if bytes.as_ptr() as usize % std::mem::align_of::<Color>() != 0 {
                panic!(
                    "alignment of color byte slice must be fitting for alignment of Color struct"
                )
            }

            std::slice::from_raw_parts(
                bytes.as_ptr() as *const Color,
                bytes.len() / std::mem::size_of::<Color>(),
            )
        };

        color_slice
    }

    /// Creates a new Color from RGBA components.
    ///
    /// # Arguments
    /// * `r` - Red component [0-255]
    /// * `g` - Green component [0-255]
    /// * `b` - Blue component [0-255]
    /// * `a` - Alpha component [0-255]
    ///
    /// # Examples
    /// ```
    /// use pixel_loop::color::Color;
    ///
    /// let color = Color::from_rgba(255, 0, 0, 255); // Opaque red
    /// ```
    pub const fn from_rgba(r: u8, b: u8, g: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new opaque Color from RGB components.
    ///
    /// # Arguments
    /// * `r` - Red component [0-255]
    /// * `g` - Green component [0-255]
    /// * `b` - Blue component [0-255]
    ///
    /// # Examples
    /// ```
    /// use pixel_loop::color::Color;
    ///
    /// let color = Color::from_rgb(255, 0, 0); // Opaque red
    /// ```
    pub const fn from_rgb(r: u8, b: u8, g: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Converts the color to a raw byte slice.
    ///
    /// # Returns
    /// A slice containing the raw RGBA bytes of the color
    pub fn as_bytes(&self) -> &[u8] {
        let color_slice = std::slice::from_ref(self);
        let byte_slice = unsafe {
            std::slice::from_raw_parts(
                color_slice.as_ptr() as *const u8,
                std::mem::size_of::<Color>(),
            )
        };
        byte_slice
    }

    /// Converts the color to HSL color space.
    ///
    /// # Returns
    /// A new HslColor representing the same color in HSL space
    ///
    /// # Examples
    /// ```
    /// use pixel_loop::color::Color;
    ///
    /// let rgb = Color::from_rgb(255, 0, 0);
    /// let hsl = rgb.as_hsl();
    /// assert_eq!(hsl.h, 0.0); // Red has hue 0
    /// assert_eq!(hsl.s, 100.0); // Full saturation
    /// assert_eq!(hsl.l, 50.0); // Mid lightness
    /// ```
    pub fn as_hsl(&self) -> HslColor {
        // Taken and converted from: https://stackoverflow.com/a/9493060
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;
        let vmax = r.max(g.max(b));
        let vmin = r.min(g.min(b));
        let l = (vmax + vmin) / 2.0;

        if vmax == vmin {
            return HslColor::new(0.0, 0.0, l); // achromatic
        }

        let d = vmax - vmin;
        let s = if l > 0.5 {
            d / (2.0 - vmax - vmin)
        } else {
            d / (vmax + vmin)
        };

        let mut h = (vmax + vmin) / 2.0;

        if vmax == r {
            h = (g - b) / d;
            if g < b {
                h += 6.0
            }
        }

        if vmax == g {
            h = (b - r) / d + 2.0;
        }

        if vmax == b {
            h = (r - g) / d + 4.0;
        }

        h /= 6.0;

        // The color conversion moves every value into the [0,1] number space.
        // But we want the hue in [0,360], s in [0,100] and l in [0,100]
        HslColor::new(h * 360f64, s * 100f64, l * 100f64)
    }
}

impl From<HslColor> for Color {
    fn from(v: HslColor) -> Self {
        // Taken and converted from: https://stackoverflow.com/a/9493060

        fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
            let mut t = t;
            if t < 0f64 {
                t += 1f64
            };
            if t > 1f64 {
                t -= 1f64
            };
            if t < 1f64 / 6f64 {
                return p + (q - p) * 6f64 * t;
            }
            if t < 1f64 / 2f64 {
                return q;
            }
            if t < 2f64 / 3f64 {
                return p + (q - p) * (2f64 / 3f64 - t) * 6f64;
            };
            return p;
        }

        let r;
        let g;
        let b;

        // The input for this algorithm expects all the h,s and l values in the
        // range [0,1].
        let h = v.h / 360f64;
        let s = v.s / 100f64;
        let l = v.l / 100f64;

        if s == 0.0 {
            r = l;
            g = l;
            b = l;
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;

            r = hue_to_rgb(p, q, h + 1f64 / 3f64);
            g = hue_to_rgb(p, q, h);
            b = hue_to_rgb(p, q, h - 1f64 / 3f64);
        }
        Color::from_rgb(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        )
    }
}

/// A color representation in HSL (Hue, Saturation, Lightness) color space.
pub struct HslColor {
    /// Hue component [0-360]
    pub h: f64,
    /// Saturation component [0-100]
    pub s: f64,
    /// Lightness component [0-100]
    pub l: f64,
}

impl HslColor {
    /// Creates a new HslColor from HSL components.
    ///
    /// # Arguments
    /// * `h` - Hue [0-360]
    /// * `s` - Saturation [0-100]
    /// * `l` - Lightness [0-100]
    ///
    /// # Examples
    ///
    /// Initialize a pure red color:
    /// ```
    /// use pixel_loop::color::HslColor;
    ///
    /// let color = HslColor::new(0.0, 100.0, 50.0); // Pure red
    /// ```
    ///
    /// Convert from RGB to HSL:
    /// ```
    /// use pixel_loop::color::{Color, HslColor};
    /// let rgb = Color::from_rgb(255, 0, 0);
    /// let hsl = rgb.as_hsl();
    /// assert_eq!(hsl.h, 0.0); // Red has hue 0
    /// assert_eq!(hsl.s, 100.0); // Full saturation
    /// assert_eq!(hsl.l, 50.0); // Mid lightness
    /// ```
    /// Convert from HSL to RGB:
    /// ```
    /// use pixel_loop::color::{Color, HslColor};
    /// let hsl = HslColor::new(0.0, 100.0, 50.0);
    /// let rgb = Color::from(hsl);
    /// assert_eq!(rgb, Color::from_rgb(255, 0, 0)); // Pure red
    /// ```
    ///
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }
}
